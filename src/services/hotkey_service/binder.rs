use std::collections::HashMap;

use dioxus::desktop::{ShortcutHandle, ShortcutRegistryError, window};
use dioxus::hooks::UnboundedSender;
use global_hotkey::HotKeyState::Pressed;
use log::warn;

use crate::models::{Action, Hotkey};
use crate::services::hotkey_service::error::HotkeyBindError;

pub trait HotkeyBinder {
    fn bind_hotkey(&mut self, hotkey: Hotkey, action: &Action) -> Result<(), HotkeyBindError>;
    fn unbind_hotkey(&mut self, hotkey: Hotkey);
}

pub struct DioxusBinder {
    hotkey_sender: UnboundedSender<(Hotkey, Action)>,
    handles: HashMap<Hotkey, ShortcutHandle>,
}

impl DioxusBinder {
    pub(super) fn new(hotkey_sender: UnboundedSender<(Hotkey, Action)>) -> Self {
        Self {
            hotkey_sender,
            handles: HashMap::new(),
        }
    }
}

impl HotkeyBinder for DioxusBinder {
    fn bind_hotkey(&mut self, hotkey: Hotkey, action: &Action) -> Result<(), HotkeyBindError> {
        let my_hotkey_sender = self.hotkey_sender.clone();
        let my_action = action.clone();
        let callback = move |state| {
            if state == Pressed {
                my_hotkey_sender
                    .unbounded_send((hotkey, my_action.clone()))
                    .unwrap();
            }
        };
        let handle = window()
            .create_shortcut(hotkey.global_hotkey(), callback)
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
            warn!("missing handle for hotkey {}", hotkey);
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::mpsc::Sender;

    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    pub enum MockEvent {
        Register(Hotkey, Action),
        Unregister(Hotkey),
    }

    pub struct MockBinder {
        pub event_sender: Sender<MockEvent>,
    }

    impl HotkeyBinder for MockBinder {
        fn bind_hotkey(&mut self, hotkey: Hotkey, action: &Action) -> Result<(), HotkeyBindError> {
            self.event_sender
                .send(MockEvent::Register(hotkey, action.clone()))
                .unwrap();
            Ok(())
        }

        fn unbind_hotkey(&mut self, hotkey: Hotkey) {
            self.event_sender
                .send(MockEvent::Unregister(hotkey))
                .unwrap();
        }
    }
}
