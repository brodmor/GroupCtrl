use global_hotkey::hotkey::Modifiers;

use crate::os::App;

pub trait KeyboardBehavior {
    fn modifier_format() -> [(Modifiers, &'static str); 4];
    fn is_multi_select(modifiers: Modifiers) -> bool;
}

pub trait AppQuery {
    fn current_app() -> anyhow::Result<Option<App>>;
}

pub trait AppSelection {
    async fn select_app() -> anyhow::Result<Option<App>>;
}

pub trait Openable {
    fn open(&self) -> anyhow::Result<()>;
}
