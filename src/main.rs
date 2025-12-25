mod components;
mod hooks;
mod models;
mod os;
mod services;
mod util;

use std::fs;

use simplelog::*;

use crate::components::Root;

fn setup_logging() -> anyhow::Result<()> {
    fs::create_dir_all("logs")?;
    let log_file = fs::File::create("logs/app.log")?;
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

    // Make panics crash loudly during development
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("PANIC: {}", panic_info);
        std::process::exit(1);
    }));

    use dioxus::desktop::{Config, WindowBuilder};

    dioxus::LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(WindowBuilder::new().with_always_on_top(false)))
        .launch(Root);
}
