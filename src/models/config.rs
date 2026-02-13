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
    pub fn bindings(&self) -> Vec<(Option<Hotkey>, Action)> {
        self.groups.iter().map(|g| g.binding()).collect()
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

    pub fn set_name(&mut self, group_id: Uuid, name: String) -> anyhow::Result<()> {
        let group = self.group_mut(group_id)?;
        group.name = name;
        Ok(())
    }

    pub fn set_hotkey(&mut self, group_id: Uuid, hotkey: Option<Hotkey>) -> anyhow::Result<()> {
        let group = self.group_mut(group_id)?;
        group.hotkey = hotkey;
        Ok(())
    }

    pub fn add_app(&mut self, group_id: Uuid, app: App) -> anyhow::Result<()> {
        let group = self.group_mut(group_id)?;
        group.add_app(app);
        Ok(())
    }

    pub fn remove_app(&mut self, group_id: Uuid, app_id: String) -> anyhow::Result<()> {
        let group = self.group_mut(group_id)?;
        group.remove_app(app_id);
        Ok(())
    }
}
