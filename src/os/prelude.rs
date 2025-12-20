use anyhow::Result;
use global_hotkey::hotkey::Modifiers;

pub trait FormatTrait {
    fn modifiers() -> [(Modifiers, &'static str); 4];
}

pub trait AppTrait {
    fn id(&self) -> &str;
    fn new(id: &str) -> Self;
    fn display(&self) -> String;
}

pub trait Openable {
    fn open(&self) -> Result<()>;
}
