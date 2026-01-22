use std::collections::HashSet;

use dioxus::desktop::window;
use dioxus::prelude::*;
use futures_util::StreamExt;
use uuid::Uuid;

use crate::components::group_config::GroupConfig;
use crate::components::lists::{GroupList, ListOperation};
use crate::models::Action;
use crate::services::{ActionService, ConfigService, SharedSender};

#[component]
pub fn Root() -> Element {
    let registered_record_sender = use_hook(SharedSender::new);
    let action_sender = use_hook(SharedSender::new);
    let config_service =
        use_signal(|| ConfigService::new(registered_record_sender.clone(), action_sender.clone()));
    // We inject the action sender like this to bypass the cyclic dependency with config service
    action_sender.set(Some(use_action_listener(config_service)));
    use_context_provider(|| registered_record_sender);
    use_context_provider(|| action_sender);

    use_effect(move || window().set_decorations(true));

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
                    groups: config_service.read().groups().clone(),
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

fn use_action_listener(config_service: Signal<ConfigService>) -> UnboundedSender<Action> {
    let listener = use_coroutine(move |mut receiver: UnboundedReceiver<Action>| async move {
        let mut action_service = ActionService::default();
        while let Some(action) = receiver.next().await {
            action_service.execute(&config_service.read(), &action)
        }
    });
    listener.tx()
}

fn use_group_list_listener(
    config_service: Signal<ConfigService>,
    selected: Signal<HashSet<Uuid>>,
    in_creation_group: Signal<Option<Uuid>>,
) {
    let handle_app_change = use_coroutine(
        move |mut receiver: UnboundedReceiver<ListOperation<Uuid>>| async move {
            while let Some(list_operation) = receiver.next().await {
                do_group_list_operation(config_service, selected, in_creation_group, list_operation)
            }
        },
    );
    use_context_provider(|| handle_app_change.tx()); // used in the (generic) list
}

fn do_group_list_operation(
    mut config_service: Signal<ConfigService>,
    mut selected: Signal<HashSet<Uuid>>,
    mut in_creation_group: Signal<Option<Uuid>>,
    list_operation: ListOperation<Uuid>,
) {
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
}
