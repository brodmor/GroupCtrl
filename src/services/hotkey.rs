pub mod binder;

use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use bimap::BiMap;
use log::info;

use crate::models::action::Action;
use crate::models::hotkey::Hotkey;
use crate::services::hotkey::binder::{DioxusBinder, HotkeyBinder};

pub struct HotkeyService<B: HotkeyBinder = DioxusBinder> {
    bindings: BiMap<Hotkey, Action>,
    binder: B,
}

impl HotkeyService<DioxusBinder> {
    pub fn new(recording: Arc<AtomicBool>) -> Self {
        Self {
            bindings: BiMap::new(),
            binder: DioxusBinder::new(recording),
        }
    }
}

impl<B: HotkeyBinder> HotkeyService<B> {
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
            self.binder.remove_shortcut(previous_hotkey);
        }
        self.binder.create_shortcut(hotkey, &action)?;
        self.bindings.insert(hotkey, action);
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use global_hotkey::hotkey::{Code, Modifiers};
    use serial_test::serial;

    use super::*;
    use crate::os::App;
    use crate::os::prelude::*;
    use crate::services::hotkey::binder::tests::MockBinder;
    use crate::services::hotkey::binder::tests::MockEvent::*;

    impl HotkeyService<MockBinder> {
        pub fn new_mock(binder: MockBinder) -> Self {
            Self {
                bindings: BiMap::new(),
                binder,
            }
        }
    }

    #[test]
    #[serial]
    fn bind_hotkey_new() {
        // Arrange
        let events = Arc::new(Mutex::new(Vec::new()));
        let binder = MockBinder {
            events: events.clone(),
        };
        let mut service = HotkeyService::new_mock(binder);
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        let action = Action::OpenApp(App::new("some-app"));

        // Act
        let result = service.bind_hotkey(hotkey, action.clone()).unwrap();

        // Assert
        assert_eq!(result, None);
        assert_eq!(*events.lock().unwrap(), vec![Register(hotkey, action)]);
    }

    #[test]
    #[serial]
    fn bind_hotkey_repeat() {
        // Arrange
        let events = Arc::new(Mutex::new(Vec::new()));
        let binder = MockBinder {
            events: events.clone(),
        };
        let mut service = HotkeyService::new_mock(binder);
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        let action = Action::OpenApp(App::new("some-app"));

        // Act
        service.bind_hotkey(hotkey, action.clone()).unwrap();
        let result = service.bind_hotkey(hotkey, action.clone()).unwrap();

        // Assert
        assert_eq!(result, None);
        assert_eq!(*events.lock().unwrap(), vec![Register(hotkey, action)]);
    }

    #[test]
    #[serial]
    fn bind_hotkey_conflict() {
        // Arrange
        let events = Arc::new(Mutex::new(Vec::new()));
        let binder = MockBinder {
            events: events.clone(),
        };
        let mut service = HotkeyService::new_mock(binder);
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        let old_action = Action::OpenApp(App::new("some-app"));
        let new_action = Action::OpenApp(App::new("some-other-app"));

        // Act
        service.bind_hotkey(hotkey, old_action.clone()).unwrap();
        let result = service.bind_hotkey(hotkey, new_action).unwrap();

        // Assert
        assert_eq!(result, Some(old_action.clone()));
        assert_eq!(*events.lock().unwrap(), vec![Register(hotkey, old_action)]);
    }

    #[test]
    #[serial]
    fn bind_hotkey_change() {
        // Arrange
        let events = Arc::new(Mutex::new(Vec::new()));
        let binder = MockBinder {
            events: events.clone(),
        };
        let mut service = HotkeyService::new_mock(binder);
        let old_hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        let new_hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyG);
        let action = Action::OpenApp(App::new("some-app"));

        // Act
        service.bind_hotkey(old_hotkey, action.clone()).unwrap();
        let result = service.bind_hotkey(new_hotkey, action.clone()).unwrap();

        // Assert
        assert_eq!(result, None);
        assert_eq!(
            *events.lock().unwrap(),
            vec![
                Register(old_hotkey, action.clone()),
                Unregister(old_hotkey),
                Register(new_hotkey, action)
            ]
        );
    }
}
