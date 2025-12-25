use bimap::BiMap;
use log::info;

use crate::models::action::Action;
use crate::models::hotkey::Hotkey;

// We need to store u32 because that's all we get from the keypress event
pub type HotkeyBinding = (u32, Option<Action>);

pub struct HotkeyService {
    bindings: BiMap<Hotkey, Action>,
}

impl Default for HotkeyService {
    fn default() -> Self {
        todo!()
    }
}

impl HotkeyService {
    /// Returns existing bind if hotkey is already in use
    pub fn bind_hotkey(
        &mut self,
        hotkey: Hotkey,
        action: Action,
    ) -> anyhow::Result<Option<Action>> {
        info!("Binding {hotkey} to '{action}'");
        if let Some(previous_action) = self.bindings.get_by_left(&hotkey) {
            if *previous_action == action {
                return Ok(None); // equivalent to registration
            }
            info!("Hotkey is already assigned to {previous_action}");
            return Ok(Some(previous_action.clone()));
        }
        if let Some((previous_hotkey, _)) = self.bindings.remove_by_right(&action) {
            // use window().remove_hotkey
            self.global_manager.unregister(previous_hotkey.0)?;
            self.binding_sender.send((previous_hotkey.id(), None))?
        }
        self.bindings.insert(hotkey, action.clone());
        // use window().add_hotkey
        self.global_manager.register(hotkey.0)?;
        self.binding_sender.send((hotkey.id(), Some(action)))?;
        Ok(None)
    }

    fn hotkeys(&self) -> Vec<GlobalHotkey> {
        self.bindings.left_values().map(|h| h.0).collect()
    }
}

#[cfg(test)]
mod tests {
    use global_hotkey::hotkey::{Code, Modifiers};
    use serial_test::serial;

    use super::*;
    use crate::os::App;
    use crate::os::prelude::*;

    #[test]
    #[serial]
    fn bind_hotkey_new() {
        // Arrange
        let (tx, rx) = channel::unbounded();
        let mut service = HotkeyService::new_with_sender(tx).unwrap();
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        let action = Action::OpenApp(App::new("some-app"));
        // Act
        let result = service.bind_hotkey(hotkey, action.clone()).unwrap();
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
        let mut service = HotkeyService::new_with_sender(tx).unwrap();
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        let action = Action::OpenApp(App::new("some-app"));
        // Act
        service.bind_hotkey(hotkey, action.clone()).unwrap();
        let result = service.bind_hotkey(hotkey, action.clone()).unwrap();
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
        let mut service = HotkeyService::new_with_sender(tx).unwrap();
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        let old_action = Action::OpenApp(App::new("some-app"));
        let new_action = Action::OpenApp(App::new("some-other-app"));
        // Act
        service.bind_hotkey(hotkey, old_action.clone()).unwrap();
        let result = service.bind_hotkey(hotkey, new_action.clone()).unwrap();
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
        let mut service = HotkeyService::new_with_sender(tx).unwrap();
        let old_hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        let new_hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyG);
        let action = Action::OpenApp(App::new("some-app"));
        // Act
        service.bind_hotkey(old_hotkey, action.clone()).unwrap();
        let result = service.bind_hotkey(new_hotkey, action.clone()).unwrap();
        // Assert
        assert_eq!(result, None);
        let old_binding = (old_hotkey.id(), Some(action.clone()));
        assert_eq!(rx.try_recv().unwrap(), old_binding);
        assert_eq!(rx.try_recv().unwrap(), (old_hotkey.id(), None)); // Unbind
        assert_eq!(rx.try_recv().unwrap(), (new_hotkey.id(), Some(action)));
        assert!(rx.try_recv().is_err()); // No dangling message
    }
}
