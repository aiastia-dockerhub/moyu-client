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

            // 桌面端：注册更新器/对话框插件 + 启动后异步检查新版本，弹窗询问是否安装
            #[cfg(desktop)]
            {
                app.handle().plugin(tauri_plugin_updater::Builder::new().build())?;
                app.handle().plugin(tauri_plugin_dialog::init())?;
                tauri::async_runtime::spawn(check_updates(app.handle().clone()));
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 检查失败一律静默（断网/无更新），不打扰使用
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
        Ok(None) => return,
        Err(e) => {
            eprintln!("[update] check failed: {e}");
            return;
        }
    };

    let msg = format!(
        "发现新版本 v{}，是否立即更新？更新完成后会自动重启。",
        update.version
    );
    let confirmed = app
        .dialog()
        .message(msg)
        .title("墨语 · 更新")
        .kind(MessageDialogKind::Info)
        .buttons(MessageDialogButtons::OkCancelCustom(
            "立即更新".to_string(),
            "下次再说".to_string(),
        ))
        .blocking_show();
    if !confirmed {
        return;
    }

    // 进度小窗：关闭窗口 = 取消（本次不安装不重启，下次启动会再次询问）
    let progress = match tauri::WebviewWindowBuilder::new(
        app,
        "update-progress",
        WebviewUrl::App("progress.html".into()),
    )
    .title("墨语 · 正在更新")
    .inner_size(340.0, 150.0)
    .resizable(false)
    .always_on_top(true)
    .build()
    {
        Ok(w) => w,
        Err(e) => {
            eprintln!("[update] progress window failed: {e}");
            return;
        }
    };

    let win = progress.clone();
    let mut done: u64 = 0;
    let bytes = match update
        .download(
            |chunk, total| {
                done += chunk as u64;
                if win.is_closed().unwrap_or(false) {
                    return;
                }
                if let Some(t) = total {
                    let pct = ((done as f64 / t as f64) * 100.0).min(100.0);
                    let _ = win.eval(&format!("window.up({pct:.0})"));
                }
            },
            || {},
        )
        .await
    {
        Ok(b) => b,
        Err(e) => {
            eprintln!("[update] download failed: {e}");
            let _ = progress.close();
            return;
        }
    };
    if win.is_closed().unwrap_or(false) {
        eprintln!("[update] cancelled: window closed, install skipped");
        return;
    }
    let _ = win.eval(
        "document.querySelector('.t').textContent='下载完成，正在安装…';window.up(100)",
    );
    if let Err(e) = update.install(bytes) {
        eprintln!("[update] install failed: {e}");
        let _ = progress.close();
        return;
    }
    app.restart();
}
