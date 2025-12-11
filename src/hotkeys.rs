use crate::action::Action;
use anyhow::Result;
use bimap::BiMap;
use crossbeam::channel;
use global_hotkey::hotkey::HotKey;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
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
                    action.execute()
                }
            }
        }
    }
}

pub struct HotkeyManager {
    bindings: BiMap<HotKey, Action>,
    global_manager: GlobalHotKeyManager,
    binding_sender: channel::Sender<HotkeyBinding>,
}

impl HotkeyManager {
    pub fn new() -> Result<Self> {
        let manager = GlobalHotKeyManager::new()?; // Can only have one at a time
        let (tx, rx) = channel::unbounded();
        thread::spawn(move || listen_for_hotkeys(rx));
        Ok(Self {
            bindings: BiMap::new(),
            global_manager: manager,
            binding_sender: tx,
        })
    }

    #[cfg(test)]
    fn new_with_sender(sender: channel::Sender<HotkeyBinding>) -> Result<Self> {
        let manager = GlobalHotKeyManager::new()?;
        Ok(Self {
            bindings: BiMap::new(),
            global_manager: manager,
            binding_sender: sender,
        })
    }

    /// Returns existing bind if hotkey is already in use
    pub fn bind_hotkey(&mut self, hotkey: HotKey, action: Action) -> Result<Option<Action>> {
        if let Some(previous_action) = self.bindings.get_by_left(&hotkey) {
            if *previous_action == action {
                return Ok(None); // equivalent to registration
            }
            return Ok(Some(previous_action.clone()));
        }
        if let Some(previous_hotkey) = self.bindings.get_by_right(&action) {
            self.unbind_hotkey(previous_hotkey)?
        }
        self.bindings.insert(hotkey, action.clone());
        self.global_manager.register(hotkey)?;
        self.binding_sender.send((hotkey.id(), Some(action)))?;
        Ok(None)
    }

    pub fn unbind_hotkey(&self, hotkey: &HotKey) -> Result<()> {
        Ok(self.binding_sender.send((hotkey.id(), None))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::App;
    use global_hotkey::hotkey::{Code, Modifiers};
    use serial_test::serial;

    #[test]
    #[serial]
    fn bind_hotkey_new() {
        // Arrange
        let (tx, rx) = channel::unbounded();
        let mut manager = HotkeyManager::new_with_sender(tx).unwrap();
        let hotkey = HotKey::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyF);
        let action = Action::OpenApp(App::new("com.apple.finder"));
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
        let hotkey = HotKey::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyF);
        let action = Action::OpenApp(App::new("com.apple.finder"));
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
        let hotkey = HotKey::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyF);
        let old_action = Action::OpenApp(App::new("com.apple.finder"));
        let new_action = Action::OpenApp(App::new("com.apple.safari"));
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
        let old_hotkey = HotKey::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyF);
        let new_hotkey = HotKey::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyG);
        let action = Action::OpenApp(App::new("com.apple.finder"));
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

    #[test]
    #[serial]
    fn unbind_hotkey() {
        // Arrange
        let (tx, rx) = channel::unbounded();
        let mut manager = HotkeyManager::new_with_sender(tx).unwrap();
        let hotkey = HotKey::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyF);
        let action = Action::OpenApp(App::new("com.apple.finder"));
        manager.bind_hotkey(hotkey, action).unwrap();
        rx.try_recv().unwrap(); // Clear the bind message
        // Act
        manager.unbind_hotkey(&hotkey).unwrap();
        // Assert
        assert_eq!(rx.try_recv().unwrap(), (hotkey.id(), None));
        assert!(rx.try_recv().is_err()); // No dangling message
    }
}
