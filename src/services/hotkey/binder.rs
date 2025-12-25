use std::collections::HashMap;

use anyhow::anyhow;
use dioxus::desktop::{ShortcutHandle, window};
use global_hotkey::HotKeyState::Pressed;

use super::callback::SharedHotkeyCallback;
use crate::models::{Action, Hotkey};

pub trait HotkeyBinder {
    fn bind_hotkey(&mut self, hotkey: Hotkey, action: &Action) -> anyhow::Result<()>;
    fn unbind_hotkey(&mut self, hotkey: Hotkey);
}

pub struct DioxusBinder {
    record_registered: SharedHotkeyCallback,
    handles: HashMap<Hotkey, ShortcutHandle>,
}

impl DioxusBinder {
    pub(super) fn new(record_registered: SharedHotkeyCallback) -> Self {
        Self {
            record_registered,
            handles: HashMap::new(),
        }
    }
}

impl HotkeyBinder for DioxusBinder {
    fn bind_hotkey(&mut self, hotkey: Hotkey, action: &Action) -> anyhow::Result<()> {
        let my_record = self.record_registered.clone();
        let my_action = action.clone();
        let callback = move |state| {
            if state == Pressed {
                if let Some(active_record_registered) = my_record.get() {
                    active_record_registered(hotkey);
                } else {
                    let _ = my_action.execute();
                }
            }
        };
        let handle = window()
            .create_shortcut(hotkey.0, callback)
            // manual error mapping because this error doesn't implement Display
            .map_err(|e| anyhow!("Failed to create shortcut: {:?}", e))?;
        self.handles.insert(hotkey, handle);
        Ok(())
    }

    fn unbind_hotkey(&mut self, hotkey: Hotkey) {
        let handle = self.handles.remove(&hotkey).unwrap();
        window().remove_shortcut(handle);
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    pub enum MockEvent {
        Register(Hotkey, Action),
        Unregister(Hotkey),
    }

    pub struct MockBinder {
        pub events: Arc<Mutex<Vec<MockEvent>>>,
    }

    impl HotkeyBinder for MockBinder {
        fn bind_hotkey(&mut self, hotkey: Hotkey, action: &Action) -> anyhow::Result<()> {
            let mut events = self.events.lock().unwrap();
            events.push(MockEvent::Register(hotkey, action.clone()));
            Ok(())
        }

        fn unbind_hotkey(&mut self, hotkey: Hotkey) {
            let mut events = self.events.lock().unwrap();
            events.push(MockEvent::Unregister(hotkey));
        }
    }
}
