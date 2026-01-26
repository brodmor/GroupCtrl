mod binder;
mod error;
mod sender;

use binder::{DioxusBinder, HotkeyBinder};
pub use error::HotkeyBindError;
pub use sender::SharedSender;

use crate::models::{Action, Config, Hotkey};

pub struct HotkeyService<B: HotkeyBinder = DioxusBinder> {
    binder: B,
}

impl HotkeyService<DioxusBinder> {
    pub fn new(
        record_registered_sender: SharedSender<Hotkey>,
        action_sender: SharedSender<Action>,
    ) -> Self {
        Self {
            binder: DioxusBinder::new(record_registered_sender, action_sender),
        }
    }
}

impl<B: HotkeyBinder> HotkeyService<B> {
    fn find_conflict(config: &Config, hotkey: Hotkey) -> Option<Action> {
        config
            .bindings()
            .into_iter()
            .find_map(|(hk, a)| (hk == Some(hotkey)).then_some(a))
    }

    pub fn bind_hotkey(
        &mut self,
        config: &Config,
        hotkey: Option<Hotkey>,
        existing_hotkey: Option<Hotkey>,
        action: Action,
    ) -> Result<(), HotkeyBindError> {
        if hotkey == existing_hotkey {
            return Ok(());
        }
        if let Some(hk) = hotkey
            && let Some(conflict) = Self::find_conflict(config, hk)
        {
            return Err(HotkeyBindError::Conflict {
                hotkey: hk,
                conflict: conflict.describe(config),
            });
        }

        self.unbind_hotkey(existing_hotkey);
        if let Some(hk) = hotkey {
            self.binder.bind_hotkey(hk, &action)?
        }
        Ok(())
    }

    pub fn unbind_hotkey(&mut self, hotkey: Option<Hotkey>) {
        if let Some(hk) = hotkey {
            self.binder.unbind_hotkey(hk);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use global_hotkey::hotkey::{Code, Modifiers};

    use super::binder::tests::MockBinder;
    use super::binder::tests::MockEvent::*;
    use super::*;
    use crate::services::hotkey_service::binder::tests::MockEvent;

    impl HotkeyService<MockBinder> {
        fn new_mock(binder: MockBinder) -> Self {
            Self { binder }
        }
    }

    fn setup_service() -> (HotkeyService<MockBinder>, Arc<Mutex<Vec<MockEvent>>>) {
        let events = Arc::new(Mutex::new(Vec::new()));
        let binder = MockBinder {
            events: events.clone(),
        };
        let service = HotkeyService::new_mock(binder);
        (service, events)
    }

    fn setup_group(name: &str, config: &mut Config, hotkey: Option<Hotkey>) -> Action {
        let group_id = config.add_group(name.to_string());
        config.set_hotkey(group_id, hotkey);
        Action::OpenGroup { group_id }
    }

    #[test]
    fn bind_hotkey_new() {
        // Arrange
        let (mut service, events) = setup_service();
        let mut config = Config::default();
        let action = setup_group("Test", &mut config, None);
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);

        // Act
        let result = service.bind_hotkey(&config, Some(hotkey), None, action.clone());

        // Assert
        assert_eq!(result, Ok(()));
        assert_eq!(*events.lock().unwrap(), vec![Register(hotkey, action)]);
    }

    #[test]
    fn bind_hotkey_repeat_none() {
        // Arrange
        let (mut service, events) = setup_service();
        let mut config = Config::default();
        let action = setup_group("Test", &mut config, None);

        // Act
        let result = service.bind_hotkey(&config, None, None, action.clone());

        // Assert
        assert_eq!(result, Ok(()));
        assert_eq!(*events.lock().unwrap(), vec![]);
    }

    #[test]
    fn bind_hotkey_repeat_some() {
        // Arrange
        let (mut service, events) = setup_service();
        let mut config = Config::default();
        let action = setup_group("Test", &mut config, None);
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);

        // Act
        let result = service.bind_hotkey(&config, Some(hotkey), Some(hotkey), action.clone());

        // Assert
        assert_eq!(result, Ok(()));
        assert_eq!(*events.lock().unwrap(), vec![]);
    }

    #[test]
    fn bind_hotkey_change() {
        // Arrange
        let (mut service, events) = setup_service();
        let old_hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        let new_hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyG);
        let mut config = Config::default();
        let action = setup_group("Test", &mut config, Some(old_hotkey));

        // Act
        let result =
            service.bind_hotkey(&config, Some(new_hotkey), Some(old_hotkey), action.clone());

        // Assert
        assert_eq!(result, Ok(()));
        assert_eq!(
            *events.lock().unwrap(),
            vec![Unregister(old_hotkey), Register(new_hotkey, action)]
        );
    }

    #[test]
    fn bind_hotkey_conflict() {
        // Arrange
        let (mut service, events) = setup_service();
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        let mut config = Config::default();
        setup_group("Fst", &mut config, Some(hotkey));
        let new_action = setup_group("Snd", &mut config, None);

        // Act
        let result = service.bind_hotkey(&config, Some(hotkey), None, new_action);

        // Assert
        assert_eq!(
            result,
            Err(HotkeyBindError::Conflict {
                hotkey,
                conflict: "open group 'Fst'".to_string()
            })
        );
        assert_eq!(*events.lock().unwrap(), vec![]);
    }

    #[test]
    fn unbind_hotkey() {
        // Arrange
        let (mut service, events) = setup_service();
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);

        // Act
        service.unbind_hotkey(Some(hotkey));

        // Assert
        assert_eq!(*events.lock().unwrap(), vec![Unregister(hotkey)]);
    }
}
