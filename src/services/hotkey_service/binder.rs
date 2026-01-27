use std::collections::HashMap;

use dioxus::desktop::{ShortcutHandle, ShortcutRegistryError, window};
use global_hotkey::HotKeyState::Pressed;
use log::warn;

use super::sender::SharedSender;
use crate::models::{Action, Hotkey};
use crate::services::hotkey_service::error::HotkeyBindError;

pub trait HotkeyBinder {
    fn bind_hotkey(&mut self, hotkey: Hotkey, action: &Action) -> Result<(), HotkeyBindError>;
    fn unbind_hotkey(&mut self, hotkey: Hotkey);
}

pub struct DioxusBinder {
    record_registered_sender: SharedSender<Hotkey>,
    action_sender: SharedSender<Action>,
    handles: HashMap<Hotkey, ShortcutHandle>,
}

impl DioxusBinder {
    pub(super) fn new(
        record_registered_sender: SharedSender<Hotkey>,
        action_sender: SharedSender<Action>,
    ) -> Self {
        Self {
            record_registered_sender,
            action_sender,
            handles: HashMap::new(),
        }
    }
}

impl HotkeyBinder for DioxusBinder {
    fn bind_hotkey(&mut self, hotkey: Hotkey, action: &Action) -> Result<(), HotkeyBindError> {
        let my_recorded_register_sender = self.record_registered_sender.clone();
        let my_action_sender = self.action_sender.clone();
        let my_action = action.clone();
        let callback = move |state| {
            if state == Pressed {
                if let Some(sender) = my_recorded_register_sender.get() {
                    let _ = sender.unbounded_send(hotkey);
                } else {
                    let _ = my_action_sender
                        .get()
                        .unwrap()
                        .unbounded_send(my_action.clone());
                }
            }
        };
        let handle = window()
            .create_shortcut(hotkey.0, callback)
            .map_err(|e| match e {
                ShortcutRegistryError::InvalidShortcut(_) => HotkeyBindError::Invalid { hotkey },
                _ => HotkeyBindError::Unknown { hotkey },
            })?;
        self.handles.insert(hotkey, handle);
        Ok(())
    }

    fn unbind_hotkey(&mut self, hotkey: Hotkey) {
        if let Some(handle) = self.handles.remove(&hotkey) {
            window().remove_shortcut(handle);
        } else {
            warn!("Missing handle for hotkey {}", hotkey);
        }
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
        fn bind_hotkey(&mut self, hotkey: Hotkey, action: &Action) -> Result<(), HotkeyBindError> {
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
