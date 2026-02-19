use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, consume_toast};
use uuid::Uuid;

use crate::os::{AppSelection, System};
use crate::services::ConfigService;
use crate::ui::app_list::AppList;
use crate::ui::util::{
    EditableText, HotkeyPicker, InputMode, ListOperation, TargetPicker, use_listener,
};

#[component]
pub fn GroupConfig(
    config_service: Signal<ConfigService>,
    group_id: Uuid,
    in_creation_group: Signal<Option<Uuid>>,
) -> Element {
    let group = use_memo(move || {
        config_service
            .read()
            .config()
            .group(group_id)
            .unwrap()
            .clone()
    });
    let name = use_memo(move || group().name.clone());
    let set_name = Callback::new(move |new_name: String| {
        if let Err(error) = config_service.write().set_name(group_id, new_name) {
            consume_toast().error(
                "Duplicate group name".to_string(),
                ToastOptions::new().description(error.to_string()),
            );
        }
    });
    let set_hotkey = move |hotkey| {
        if let Err(error) = config_service.write().set_hotkey(group_id, hotkey) {
            consume_toast().error(
                "Error binding hotkey".to_string(),
                ToastOptions::new().description(error.to_string()),
            );
        }
    };
    let set_target = Callback::new(move |app| {
        config_service.write().set_target(group_id, app);
    });
    use_app_list_listener(config_service, group_id);

    let list_operation_tx = use_coroutine_handle::<ListOperation<Uuid>>();
    let on_cancel = Callback::new(move |_| {
        list_operation_tx.send(ListOperation::Remove(group_id));
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
            class: "flex flex-col gap-2 flex-1 min-h-0 min-w-0 p-2",
            div {
                class: "text-sm grid items-center gap-2",
                style: "grid-template-columns: auto minmax(0, 1fr);",
                label { r#for: "editable-text", "Name" }
                EditableText {
                    text: name,
                    placeholder: "Group name".to_string(),
                    starting_mode: input_mode(),
                    on_commit: set_name,
                }
                label { r#for: "hotkey-picker", "Hotkey" }
                HotkeyPicker { hotkey: group().hotkey, set_hotkey }
                label { r#for: "target-picker", "Target" }
                TargetPicker {
                    apps: group().apps().to_vec(),
                    target: group().target.clone(),
                    set_target: set_target,
                }
            }
            AppList { apps: group().apps().to_vec() }
        }
    }
}

fn use_app_list_listener(mut config_service: Signal<ConfigService>, group_id: Uuid) {
    use_listener(Callback::new(move |list_operation| match list_operation {
        ListOperation::Add => {
            spawn(async move {
                if let Ok(Some(app)) = System::select_app().await {
                    config_service.write().add_app(group_id, app)
                }
            });
        }
        ListOperation::Remove(app_id) => {
            config_service.write().remove_app(group_id, app_id);
        }
    }));
}
