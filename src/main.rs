mod components;
mod models;
mod os;
mod services;
mod util;

use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
use simplelog::*;

use crate::components::Root;

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

    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_window(
                WindowBuilder::new()
                    .with_title("GroupCtrl")
                    .with_inner_size(LogicalSize::new(400, 300))
                    .with_always_on_top(false),
            ),
        )
        .launch(Root);
}
