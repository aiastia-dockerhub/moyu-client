use tauri::{WebviewUrl, WebviewWindowBuilder};

pub fn run() {
    // 站点地址由 CI 从 secret MOYU_URL 编译期注入，仓库内不出现域名
    let raw = match option_env!("MOYU_URL") {
        Some(u) => u,
        None => {
            eprintln!("缺少 MOYU_URL 编译参数（由 CI secret 注入）");
            std::process::exit(1);
        }
    };
    let url: tauri::Url = raw.parse().expect("MOYU_URL 无法解析");

    tauri::Builder::default()
        .setup(move |app| {
            let builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::External(url.clone()))
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
