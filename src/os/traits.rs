use std::path::PathBuf;

use global_hotkey::hotkey::Modifiers;

use crate::os::App;

pub type ModifierFormat = [(Modifiers, &'static str); 4];

pub trait KeyboardBehavior {
    fn serde_modifier_format() -> ModifierFormat;
    fn gui_modifier_format() -> ModifierFormat;
    fn key_sep() -> &'static str;
    fn is_multi_select(modifiers: Modifiers) -> bool;
}

pub trait AppQuery {
    fn current_app() -> anyhow::Result<Option<App>>;
}

pub trait AppSelection {
    async fn select_app() -> anyhow::Result<Option<App>>;
}

pub trait Openable {
    async fn open(&self) -> anyhow::Result<()>;
}

pub trait ConfigDir {
    fn config_dir() -> PathBuf;
}

pub trait AppObserver {
    fn observe_app_activations() -> std::sync::mpsc::Receiver<String>;
}
