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
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
