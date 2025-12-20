use super::model::Hotkey;
use crate::action::Action;
use anyhow::Result;
use bimap::BiMap;
use crossbeam::channel;
use global_hotkey::hotkey::HotKey as GlobalHotkey;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use log::{error, info};
use std::collections::HashMap;
use std::thread;

// We need to store u32 because that's all we get from the keypress event
pub type HotkeyBinding = (u32, Option<Action>);

pub fn listen_for_hotkeys(binding_receiver: channel::Receiver<HotkeyBinding>) {
    let mut hotkey_actions = HashMap::new();
    let hotkey_receiver = GlobalHotKeyEvent::receiver();
    loop {
        crossbeam::select! {
            recv(binding_receiver) -> binding => {
                if let Ok((hotkey, action_option)) = binding {
                    match action_option {
                        Some(action) => { hotkey_actions.insert(hotkey, action); }
                        None => { hotkey_actions.remove(&hotkey); }
                    }
                }
            }
            recv(hotkey_receiver) -> event_result => {
                if let Ok(event) = event_result
                    && event.state == global_hotkey::HotKeyState::Pressed
                    && let Some(action) = hotkey_actions.get(&event.id)
                {
                    let result = action.execute();
                    if let Err(error) = result {
                        error!("{error}");
                    }
                }
            }
        }
    }
}

pub struct HotkeyManager {
    bindings: BiMap<Hotkey, Action>,
    global_manager: GlobalHotKeyManager,
    binding_sender: channel::Sender<HotkeyBinding>,
}

impl Default for HotkeyManager {
    fn default() -> Self {
        let (tx, rx) = channel::unbounded();
        thread::spawn(move || listen_for_hotkeys(rx));
        Self {
            bindings: BiMap::new(),
            global_manager: GlobalHotKeyManager::new()
                .expect("global-hotkey manager creation failed"),
            binding_sender: tx,
        }
    }
}

impl HotkeyManager {
    #[cfg(test)]
    fn new_with_sender(sender: channel::Sender<HotkeyBinding>) -> Result<Self> {
        let global_manager = GlobalHotKeyManager::new()?;
        Ok(Self {
            bindings: BiMap::new(),
            global_manager,
            binding_sender: sender,
        })
    }

    /// Returns existing bind if hotkey is already in use
    pub fn bind_hotkey(&mut self, hotkey: Hotkey, action: Action) -> Result<Option<Action>> {
        info!("Binding {hotkey} to '{action}'");
        if let Some(previous_action) = self.bindings.get_by_left(&hotkey) {
            if *previous_action == action {
                return Ok(None); // equivalent to registration
            }
            info!("Hotkey is already assigned to {previous_action}");
            return Ok(Some(previous_action.clone()));
        }
        if let Some((previous_hotkey, _)) = self.bindings.remove_by_right(&action) {
            self.global_manager.unregister(previous_hotkey.0)?;
            self.binding_sender.send((previous_hotkey.id(), None))?
        }
        self.bindings.insert(hotkey, action.clone());
        self.global_manager.register(hotkey.0)?;
        self.binding_sender.send((hotkey.id(), Some(action)))?;
        Ok(None)
    }

    fn hotkeys(&self) -> Vec<GlobalHotkey> {
        self.bindings.left_values().map(|h| h.0).collect()
    }

    pub fn pause_hotkeys(&self) -> Result<()> {
        self.global_manager.unregister_all(&self.hotkeys())?;
        Ok(())
    }

    pub fn unpause_hotkeys(&self) -> Result<()> {
        self.global_manager.register_all(&self.hotkeys())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os::App;
    use crate::os::prelude::*;
    use global_hotkey::hotkey::{Code, Modifiers};
    use serial_test::serial;

    #[test]
    #[serial]
    fn bind_hotkey_new() {
        // Arrange
        let (tx, rx) = channel::unbounded();
        let mut manager = HotkeyManager::new_with_sender(tx).unwrap();
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        let action = Action::OpenApp(App::new("some-app"));
        // Act
        let result = manager.bind_hotkey(hotkey, action.clone()).unwrap();
        // Assert
        assert_eq!(result, None);
        assert_eq!(rx.try_recv().unwrap(), (hotkey.id(), Some(action)));
        assert!(rx.try_recv().is_err()); // No dangling message
    }

    #[test]
    #[serial]
    fn bind_hotkey_repeat() {
        // Arrange
        let (tx, rx) = channel::unbounded();
        let mut manager = HotkeyManager::new_with_sender(tx).unwrap();
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        let action = Action::OpenApp(App::new("some-app"));
        // Act
        manager.bind_hotkey(hotkey, action.clone()).unwrap();
        let result = manager.bind_hotkey(hotkey, action.clone()).unwrap();
        // Assert
        assert_eq!(result, None);
        assert_eq!(rx.try_recv().unwrap(), (hotkey.id(), Some(action)));
        assert!(rx.try_recv().is_err()); // No repeat message
    }

    #[test]
    #[serial]
    fn bind_hotkey_conflict() {
        // Arrange
        let (tx, rx) = channel::unbounded();
        let mut manager = HotkeyManager::new_with_sender(tx).unwrap();
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        let old_action = Action::OpenApp(App::new("some-app"));
        let new_action = Action::OpenApp(App::new("some-other-app"));
        // Act
        manager.bind_hotkey(hotkey, old_action.clone()).unwrap();
        let result = manager.bind_hotkey(hotkey, new_action.clone()).unwrap();
        // Assert
        assert_eq!(result, Some(old_action.clone()));
        assert_eq!(rx.try_recv().unwrap(), (hotkey.id(), Some(old_action)));
        assert!(rx.try_recv().is_err()); // No repeat message
    }

    #[test]
    #[serial]
    fn bind_hotkey_change() {
        // Arrange
        let (tx, rx) = channel::unbounded();
        let mut manager = HotkeyManager::new_with_sender(tx).unwrap();
        let old_hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        let new_hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyG);
        let action = Action::OpenApp(App::new("some-app"));
        // Act
        manager.bind_hotkey(old_hotkey, action.clone()).unwrap();
        let result = manager.bind_hotkey(new_hotkey, action.clone()).unwrap();
        // Assert
        assert_eq!(result, None);
        let old_binding = (old_hotkey.id(), Some(action.clone()));
        assert_eq!(rx.try_recv().unwrap(), old_binding);
        assert_eq!(rx.try_recv().unwrap(), (old_hotkey.id(), None)); // Unbind
        assert_eq!(rx.try_recv().unwrap(), (new_hotkey.id(), Some(action)));
        assert!(rx.try_recv().is_err()); // No dangling message
    }
}
