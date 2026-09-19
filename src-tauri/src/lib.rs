use tauri::{WebviewUrl, WebviewWindowBuilder};

// 前端(Nuxt SPA 静态包)内嵌于安装包，页面加载本地资源秒开；
// API/WS 地址在前端构建时烤入（见 workflow 的 NUXT_PUBLIC_API_BASE = MOYU_URL secret）
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let builder = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::App("index.html".into()),
            )
            .title("墨语")
            .inner_size(1440.0, 960.0)
            .min_inner_size(1024.0, 700.0);
            // center() 仅桌面端有
            #[cfg(desktop)]
            let builder = builder.center();
            builder.build()?;

            // 桌面端：注册更新器 + 启动后异步检查新版本，弹窗询问是否安装
            #[cfg(desktop)]
            {
                app.handle().plugin(tauri_plugin_updater::Builder::new().build())?;
                tauri::async_runtime::spawn(check_updates(app.handle().clone()));
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 检查失败一律静默（私库/断网/无更新），不打扰使用
#[cfg(desktop)]
async fn check_updates(app: tauri::AppHandle) {
    use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
    use tauri_plugin_updater::UpdaterExt;

    let updater = match app.updater() {
        Ok(u) => u,
        Err(e) => {
            eprintln!("[update] updater init failed: {e}");
            return;
        }
    };
    let update = match updater.check().await {
        Ok(Some(u)) => {
            eprintln!("[update] found v{}", u.version);
            u
        }
        Ok(None) => {
            eprintln!("[update] no update available");
            return;
        }
        Err(e) => {
            eprintln!("[update] check failed: {e}");
            return;
        }
    };

    let msg = format!(
        "发现新版本 v{}，是否下载并安装？安装后会自动重启。",
        update.version
    );
    let confirmed = app
        .dialog()
        .message(msg)
        .title("墨语 · 更新")
        .kind(MessageDialogKind::Info)
        .blocking_show();
    eprintln!("[update] dialog confirmed: {confirmed}");
    if !confirmed {
        return;
    }
    let mut downloaded: u64 = 0;
    let mut finished = false;
    let _ = update
        .download_and_install(
            |chunk, total| {
                downloaded += chunk as u64;
                if total.unwrap_or(0) > 0 {
                    println!("更新下载 {downloaded}/{total:?}");
                }
            },
            || finished = true,
        )
        .await;
    if finished {
        app.restart();
    }
}
