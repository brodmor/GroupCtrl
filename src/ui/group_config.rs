use dioxus::prelude::*;
use uuid::Uuid;

use crate::os::{AppSelection, System};
use crate::services::ConfigService;
use crate::ui::lists::{AppList, ListOperation};
use crate::ui::util::{
    EditableText, HotkeyPicker, InputMode, LabeledRow, MainAppPicker, use_listener,
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
    let mut set_hotkey_result = use_signal(|| Ok(()));
    let set_hotkey = move |hotkey| {
        let result = config_service.write().set_hotkey(group_id, hotkey);
        set_hotkey_result.set(result);
    };
    let set_main_app = Callback::new(move |app| {
        config_service.write().set_main_app(group_id, app);
    });
    let name = use_signal(|| group().name.clone());
    use_effect(move || config_service.write().set_name(group_id, name()));
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
            class: "flex flex-col gap-2",
            EditableText {
                text: name,
                placeholder: "Group name".to_string(),
                starting_mode: input_mode()
            }
            LabeledRow {
                label: "On".to_string(),
                HotkeyPicker { hotkey: group().hotkey, set_hotkey },
            }
            if let Err(error) = set_hotkey_result() {
                span {
                    class: "text-xs text-error",
                    "{error}"
                }
            }
            LabeledRow {
                label: "Open".to_string(),
                MainAppPicker {
                    apps: group().apps().to_vec(),
                    main_app: group().main_app().cloned(),
                    set_main_app: set_main_app,
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
