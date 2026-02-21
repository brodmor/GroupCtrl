#[allow(dead_code, unused_imports)]
mod components;
mod models;
mod os;
mod services;
mod ui;
mod util;

use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
use dioxus::prelude::*;
use simplelog::*;

use crate::os::{AppQuery, System};
use crate::ui::Root;

const FONT_URL: &str = "https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&display=swap";

#[cfg(all(debug_assertions, target_os = "macos"))]
pub static PREVIOUS_APP: std::sync::OnceLock<String> = std::sync::OnceLock::new();

fn setup_logging() -> anyhow::Result<()> {
    std::fs::create_dir_all(os::logs_dir())?;
    let log_file = std::fs::File::create(os::logs_dir().join("app.log"))?;
    let config = ConfigBuilder::new().build();
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::Debug, config, log_file),
    ])?;
    Ok(())
}

fn main() {
    setup_logging().expect("Logging setup failed");

    if os::icons_dir().exists() {
        let _ = std::fs::remove_dir_all(os::icons_dir());
    }

    std::panic::set_hook(Box::new(|panic_info| {
        log::error!("PANIC: {}", panic_info);
        std::process::exit(1);
    }));

    #[cfg(target_os = "macos")]
    let head = format!(
        r#"<link rel="stylesheet" href="{FONT_URL}"><link rel="stylesheet" href="{}">"#,
        asset!("/assets/tailwind.css")
    );
    #[cfg(target_os = "windows")]
    let head = format!(
        r#"<link rel="stylesheet" href="{FONT_URL}"><style>{}</style>"#,
        include_str!("../assets/tailwind.css")
    );

    #[cfg(all(debug_assertions, target_os = "macos"))]
    if let Ok(Some(id)) = System::current_app() {
        let _ = PREVIOUS_APP.set(id);
    }

    let window = {
        let max_size = if cfg!(debug_assertions) {
            LogicalSize::new(1200, 800)
        } else {
            LogicalSize::new(600, 600)
        };
        let builder = WindowBuilder::new()
            .with_transparent(true)
            .with_decorations(false)
            .with_inner_size(LogicalSize::new(500, 400))
            .with_min_inner_size(LogicalSize::new(400, 400))
            .with_max_inner_size(max_size);
        #[cfg(debug_assertions)] // for hot reload
        let builder = builder.with_always_on_top(true).with_focused(false);
        builder.with_title("GroupCtrl")
    };

    LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(window).with_custom_head(head))
        .launch(Root);
}
