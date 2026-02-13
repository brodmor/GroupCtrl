use std::collections::HashSet;
use std::sync::{Arc, RwLock};

use dioxus::desktop::window;
use dioxus::prelude::*;
use uuid::Uuid;

use crate::components::group_config::GroupConfig;
use crate::components::lists::{GroupList, ListOperation};
use crate::components::util::use_listener;
use crate::models::{Config, Hotkey};
use crate::services::{ActionService, ConfigReader, ConfigService};

#[component]
pub fn Root() -> Element {
    use_effect(move || window().set_decorations(true));

    let config_service = use_config_service();
    let selected = use_signal(HashSet::<Uuid>::new);
    let in_creation_group = use_signal(|| None::<Uuid>);
    use_group_list_listener(config_service, selected, in_creation_group);

    let active_group = use_memo(move || {
        if selected().len() == 1 {
            selected().iter().next().copied()
        } else {
            None
        }
    });

    rsx! {
        div {
            "data-theme": "dim",
            class: "flex h-screen",
            aside {
                class: "flex-1 p-2 border-r",
                GroupList {
                    groups: config_service.read().config().groups().clone(),
                    selected
                }
            }
            main {
                class: "flex-1 p-2",
                if let Some(group_id) = active_group() {
                    GroupConfig {
                        key: "{group_id}",
                        config_service,
                        group_id,
                        in_creation_group
                    }
                }
            }
        }
    }
}

fn use_config_service() -> Signal<ConfigService> {
    let config = Arc::new(RwLock::new(Config::load().unwrap_or_default()));
    let config_reader = ConfigReader::new(config.clone());
    let action_service = ActionService::new(config_reader);

    let active_recorder = use_context_provider(|| Signal::new(None::<UnboundedSender<Hotkey>>));
    let hotkey_sender = use_listener(Callback::new(move |(hotkey, action)| {
        if let Some(sender) = active_recorder() {
            sender.unbounded_send(hotkey).unwrap();
        } else {
            let service = action_service.clone();
            spawn(async move {
                service.execute(&action).await;
            });
        }
    }));

    use_signal(|| ConfigService::new(config, hotkey_sender))
}

fn use_group_list_listener(
    mut config_service: Signal<ConfigService>,
    mut selected: Signal<HashSet<Uuid>>,
    mut in_creation_group: Signal<Option<Uuid>>,
) {
    use_listener(Callback::new(move |list_operation: ListOperation<Uuid>| {
        selected.write().clear();
        match list_operation {
            ListOperation::Add => {
                let group_id = config_service.write().add_group("New Group".to_string());
                selected.write().insert(group_id);
                in_creation_group.set(Some(group_id));
            }
            ListOperation::Remove(groups) => {
                for group_id in groups {
                    config_service.write().remove_group(group_id);
                }
            }
        }
    }));
}
