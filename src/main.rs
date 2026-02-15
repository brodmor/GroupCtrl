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

use crate::ui::Root;

fn setup_logging() -> anyhow::Result<()> {
    std::fs::create_dir_all("logs")?;
    let log_file = std::fs::File::create("logs/app.log")?;
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

    #[cfg(debug_assertions)] // Make panics crash loudly
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("PANIC: {}", panic_info);
        std::process::exit(1);
    }));

    #[cfg(target_os = "macos")]
    let head = format!(
        r#"<link rel="stylesheet" href="{}"><link rel="stylesheet" href="{}">"#,
        asset!("/assets/tailwind.css"),
        asset!("/assets/dx-components-theme.css")
    );
    #[cfg(target_os = "windows")]
    let head = format!(
        r#"<style>{}{}</style>"#,
        include_str!("../assets/tailwind.css"),
        include_str!("../assets/dx-components-theme.css")
    );

    LaunchBuilder::desktop()
        .with_cfg(
            Config::new()
                .with_window(
                    WindowBuilder::new()
                        .with_decorations(false)
                        .with_always_on_top(false)
                        .with_inner_size(LogicalSize::new(400, 300))
                        .with_title("GroupCtrl"),
                )
                .with_custom_head(head),
        )
        .launch(Root);
}
