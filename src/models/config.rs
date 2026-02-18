use anyhow::Context;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::group::Group;
use crate::models::hotkey::Hotkey;
use crate::models::{Action, Bindable, Identifiable};
use crate::os::App;

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct Config {
    groups: Vec<Group>,
    // settings: Settings as enum (can implement Actionable)
}

impl Config {
    pub fn bindings(&self) -> Vec<(Hotkey, Action)> {
        self.groups
            .iter()
            .filter_map(|group| {
                let (hotkey, action) = group.binding();
                Some((hotkey?, action))
            })
            .collect()
    }

    pub fn groups(&self) -> &Vec<Group> {
        &self.groups
    }

    pub fn add_group(&mut self, name: String) -> Uuid {
        let group = Group::new(name);
        let group_id = group.id();
        self.groups.push(group);
        group_id
    }

    pub fn remove_group(&mut self, group_id: Uuid) {
        self.groups.retain(|g| g.id() != group_id)
    }

    pub fn group(&self, group_id: Uuid) -> anyhow::Result<&Group> {
        self.groups
            .iter()
            .find(|g| g.id() == group_id)
            .with_context(|| format!("group {} not found", group_id))
    }

    fn group_mut(&mut self, group_id: Uuid) -> anyhow::Result<&mut Group> {
        self.groups
            .iter_mut()
            .find(|g| g.id() == group_id)
            .with_context(|| format!("group {} not found (mut)", group_id))
    }

    pub fn set_name(&mut self, group_id: Uuid, name: String) -> bool {
        if self
            .groups
            .iter()
            .any(|g| g.id() != group_id && g.name == name)
        {
            return false;
        }
        self.group_mut(group_id).unwrap().name = name;
        true
    }

    pub fn set_hotkey(&mut self, group_id: Uuid, hotkey: Option<Hotkey>) {
        self.group_mut(group_id).unwrap().hotkey = hotkey;
    }

    pub fn set_target(&mut self, group_id: Uuid, app: Option<App>) {
        self.group_mut(group_id).unwrap().target = app;
    }

    pub fn add_app(&mut self, group_id: Uuid, app: App) {
        self.group_mut(group_id).unwrap().add_app(app);
    }

    pub fn remove_app(&mut self, group_id: Uuid, app_id: String) {
        self.group_mut(group_id).unwrap().remove_app(app_id);
    }
}
