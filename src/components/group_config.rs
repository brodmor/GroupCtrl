use std::collections::HashSet;

use dioxus::prelude::*;
use uuid::Uuid;

use crate::components::lists::{AppList, ListOperation};
use crate::components::util::{EditableText, HotkeyPicker, InputMode, spawn_listener};
use crate::os::{AppSelection, System};
use crate::services::ConfigService;

#[component]
pub fn GroupConfig(
    config_service: Signal<ConfigService>,
    group_id: Uuid,
    in_creation_group: Signal<Option<Uuid>>,
) -> Element {
    let group = use_memo(move || config_service.read().group(group_id).unwrap().clone());
    let mut set_hotkey_result = use_signal(|| Ok(()));
    let set_hotkey = move |hotkey| {
        let result = config_service.write().set_hotkey(group_id, hotkey);
        set_hotkey_result.set(result);
    };
    let name = use_signal(|| group().name.clone());
    use_effect(move || config_service.write().set_name(group_id, name()));
    let app_list_sender = use_app_list_listener(config_service, group_id);
    use_context_provider(|| app_list_sender);

    let list_operation_sender = use_context::<UnboundedSender<ListOperation<Uuid>>>();

    let on_cancel = EventHandler::new(move |_| {
        let selected = HashSet::from([group_id]);
        let _ = list_operation_sender.unbounded_send(ListOperation::Remove(selected));
    });

    let input_mode = use_signal(|| {
        if in_creation_group() == Some(group_id) {
            in_creation_group.set(None); // Consume signal so it doesn't persist
            InputMode::Create { on_cancel }
        } else {
            InputMode::Edit
        }
    });

    rsx! {
        div {
            class: "flex flex-col gap-2",
            EditableText {
                text: name,
                placeholder: "Group name".to_string(),
                starting_mode: input_mode()
            }
            HotkeyPicker { hotkey: group().hotkey, set_hotkey },
            if let Err(error) = set_hotkey_result() {
                span {
                    class: "text-xs text-error",
                    "{error}"
                }
            }
            AppList { apps: group().apps().to_vec() }
        }
    }
}

fn use_app_list_listener(
    mut config_service: Signal<ConfigService>,
    group_id: Uuid,
) -> UnboundedSender<ListOperation<String>> {
    spawn_listener(EventHandler::new(
        move |list_operation| match list_operation {
            ListOperation::Add => {
                spawn(async move {
                    if let Ok(Some(app)) = System::select_app().await {
                        config_service.write().add_app(group_id, app)
                    }
                });
            }
            ListOperation::Remove(apps) => {
                for app_id in apps {
                    config_service.write().remove_app(group_id, app_id);
                }
            }
        },
    ))
}
