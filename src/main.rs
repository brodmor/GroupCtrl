mod action;
mod app;
mod hotkeys;
mod open;
mod util;

use crate::action::Action;
use crate::app::App;
use crate::hotkeys::HotkeyManager;
use anyhow::Result;
use eframe::egui;
use eframe::egui::Button;
use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use std::fs;
use std::fs::File;

use simplelog::*;

struct GroupCtrl {
    hotkey_manager: HotkeyManager,
    error: Option<String>,
}

impl GroupCtrl {
    fn new() -> Self {
        Self {
            hotkey_manager: HotkeyManager::new().unwrap(),
            error: None,
        }
    }
    fn handle_register_click(&mut self, hotkey: HotKey, app: App) {
        let result = self
            .hotkey_manager
            .bind_hotkey(hotkey, Action::OpenApp(app));
        match result {
            Err(err) => {
                self.error = Some(err.to_string());
            }
            Ok(action_option) => {
                // this is pointless atm, for future use
                self.error = None;
                if let Some(action) = action_option {
                    // TODO popup
                    let msg = format!("Hotkey already in use for '{}'", action);
                    self.error = Some(msg);
                }
            }
        }
    }
}

impl eframe::App for GroupCtrl {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let app = App::new("com.apple.finder");
            let hotkey = HotKey::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyF);
            ui.horizontal(|ui| {
                ui.label(app.to_string());
                ui.label(hotkey.to_string())
            });
            let button = Button::new("Register hotkey");
            if ui.add(button).clicked() {
                self.handle_register_click(hotkey, app);
            }
            if let Some(error) = &self.error {
                ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
            }
        });
    }
}

fn setup_logging() -> Result<()> {
    fs::create_dir_all("logs")?;
    let log_file = File::create("logs/app.log")?;
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

fn main() -> eframe::Result<()> {
    setup_logging().expect("Logging setup failed");
    eframe::run_native(
        "GroupCtrl",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(GroupCtrl::new()))),
    )
}
