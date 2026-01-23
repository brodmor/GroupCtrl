use dioxus::prelude::*;
use futures_util::StreamExt;
use uuid::Uuid;

use crate::components::lists::{AppList, ListOperation};
use crate::components::util::{EditableText, HotkeyPicker, InputMode};
use crate::os::{AppSelection, System};
use crate::services::ConfigService;

#[component]
pub fn GroupConfig(
    config_service: Signal<ConfigService>,
    group_id: Uuid,
    in_creation_group: Signal<Option<Uuid>>,
) -> Element {
    let group = use_memo(move || config_service.read().group(group_id).unwrap().clone());
    let set_hotkey = move |hotkey| {
        // TODO this can fail
        config_service.write().set_hotkey(group_id, hotkey);
    };
    let name = use_signal(|| group().name.clone());
    use_effect(move || config_service.write().set_name(group_id, name()));
    use_app_list_listener(config_service, group_id);

    let list_operation_sender = use_context::<UnboundedSender<ListOperation<Uuid>>>();
    let input_mode = use_signal(|| {
        if in_creation_group() == Some(group_id) {
            in_creation_group.set(None); // Consume signal so it doesn't persist
            let on_cancel = EventHandler::new(move |_| {
                let selected = [group_id].into_iter().collect();
                let _ = list_operation_sender.unbounded_send(ListOperation::Remove(selected));
            });
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
            HotkeyPicker { hotkey: group().hotkey, set_hotkey }
            AppList { apps: group().apps().to_vec() }
        }
    }
}

fn use_app_list_listener(config_service: Signal<ConfigService>, group_id: Uuid) {
    let app_list_listener = use_coroutine(
        move |mut receiver: UnboundedReceiver<ListOperation<String>>| async move {
            while let Some(list_operation) = receiver.next().await {
                do_app_list_operation(config_service, group_id, list_operation).await;
            }
        },
    );
    use_context_provider(|| app_list_listener.tx()); // used in the (generic) list
}

async fn do_app_list_operation(
    mut config_service: Signal<ConfigService>,
    group_id: Uuid,
    list_operation: ListOperation<String>,
) {
    match list_operation {
        ListOperation::Add => {
            if let Ok(Some(app)) = System::select_app().await {
                config_service.write().add_app(group_id, app)
            }
        }
        ListOperation::Remove(apps) => {
            for app_id in apps {
                config_service.write().remove_app(group_id, app_id);
            }
        }
    }
}
