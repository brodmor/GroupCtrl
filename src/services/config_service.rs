use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use dioxus::hooks::UnboundedSender;
use uuid::Uuid;

use crate::models::{Action, Bindable, Config, Hotkey};
use crate::os::App;
use crate::services::HotkeyService;
use crate::services::config_reader::ConfigReader;
use crate::services::hotkey_service::HotkeyBindError;

pub struct ConfigService {
    config: Arc<RwLock<Config>>,
    hotkey_service: HotkeyService,
}

impl ConfigService {
    pub fn new(
        config: Arc<RwLock<Config>>,
        hotkey_sender: UnboundedSender<(Hotkey, Action)>,
    ) -> Self {
        Self {
            config: config.clone(),
            hotkey_service: HotkeyService::new(ConfigReader::new(config), hotkey_sender),
        }
    }

    // This is reactive and intended for usage in Dioxus
    pub fn config(&self) -> RwLockReadGuard<'_, Config> {
        self.config.read().unwrap()
    }

    fn config_mut(&self) -> RwLockWriteGuard<'_, Config> {
        self.config.write().unwrap()
    }

    fn save(&self) {
        self.config().save().unwrap();
    }

    pub fn add_group(&mut self, name: String) -> Uuid {
        let id = self.config_mut().add_group(name);
        self.save();
        id
    }

    pub fn remove_group(&mut self, group_id: Uuid) {
        let hotkey = self.config().group(group_id).unwrap().hotkey;
        self.hotkey_service.unbind_hotkey(hotkey);
        self.config_mut().remove_group(group_id);
        self.save();
    }

    pub fn set_name(&mut self, group_id: Uuid, name: String) -> bool {
        let set = self.config_mut().set_name(group_id, name);
        if set {
            self.save();
        }
        set
    }

    pub fn set_target(&mut self, group_id: Uuid, app: Option<App>) {
        self.config_mut().set_target(group_id, app);
        self.save();
    }

    pub fn add_app(&mut self, group_id: Uuid, app: App) {
        self.config_mut().add_app(group_id, app);
        self.save();
    }

    pub fn remove_app(&mut self, group_id: Uuid, app_id: String) {
        self.config_mut().remove_app(group_id, app_id);
        self.save();
    }

    pub fn set_hotkey(
        &mut self,
        group_id: Uuid,
        hotkey: Option<Hotkey>,
    ) -> Result<(), HotkeyBindError> {
        let (existing_hotkey, action) = self.config().group(group_id).unwrap().binding();
        self.hotkey_service
            .bind_hotkey(hotkey, existing_hotkey, action)?;
        self.config_mut().set_hotkey(group_id, hotkey);
        self.save();
        Ok(())
    }
}
