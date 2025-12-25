use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::anyhow;
use dioxus::desktop::{ShortcutHandle, window};
use global_hotkey::HotKeyState::Pressed;

use crate::models::action::Action;
use crate::models::hotkey::Hotkey;

pub trait HotkeyBinder {
    fn create_shortcut(&mut self, hotkey: Hotkey, action: &Action) -> anyhow::Result<()>;
    fn remove_shortcut(&mut self, hotkey: Hotkey);
}

pub struct DioxusBinder {
    recording: Arc<AtomicBool>,
    handles: HashMap<Hotkey, ShortcutHandle>,
}

impl DioxusBinder {
    pub fn new(recording: Arc<AtomicBool>) -> Self {
        Self {
            recording,
            handles: HashMap::new(),
        }
    }
}

impl HotkeyBinder for DioxusBinder {
    fn create_shortcut(&mut self, hotkey: Hotkey, action: &Action) -> anyhow::Result<()> {
        let my_action = action.clone();
        let my_recording = self.recording.clone();
        let callback = move |state| {
            if state == Pressed && !my_recording.load(Ordering::SeqCst) {
                let _ = my_action.execute();
            }
        };
        let handle = window()
            .create_shortcut(hotkey.0, callback)
            // manual error mapping because this error doesn't implement Display
            .map_err(|e| anyhow!("Failed to create shortcut: {:?}", e))?;
        self.handles.insert(hotkey, handle);
        Ok(())
    }

    fn remove_shortcut(&mut self, hotkey: Hotkey) {
        let handle = self.handles.remove(&hotkey).unwrap();
        window().remove_shortcut(handle);
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::Mutex;

    use super::*;
    use crate::models::hotkey::Hotkey;

    #[derive(Debug, PartialEq, Clone)]
    pub enum MockEvent {
        Register(Hotkey, Action),
        Unregister(Hotkey),
    }

    pub struct MockBinder {
        pub events: Arc<Mutex<Vec<MockEvent>>>,
    }

    impl HotkeyBinder for MockBinder {
        fn create_shortcut(&mut self, hotkey: Hotkey, action: &Action) -> anyhow::Result<()> {
            let mut events = self.events.lock().unwrap();
            events.push(MockEvent::Register(hotkey, action.clone()));
            Ok(())
        }

        fn remove_shortcut(&mut self, hotkey: Hotkey) {
            let mut events = self.events.lock().unwrap();
            events.push(MockEvent::Unregister(hotkey));
        }
    }
}
