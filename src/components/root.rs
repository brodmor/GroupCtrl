use dioxus::prelude::*;

use super::app_selector::AppSelector;
use super::hotkey_picker::HotkeyPicker;
use crate::models::{Action, Hotkey};
use crate::os::App;
use crate::services::{HotkeyService, SharedHotkeyCallback};

#[component]
pub fn Root() -> Element {
    let record_registered = use_hook(SharedHotkeyCallback::default);
    let mut hotkey_service = use_signal(|| HotkeyService::new(record_registered.clone()));
    use_context_provider(|| record_registered); // provide to hotkey pickers

    let picked_hotkey = use_signal(|| None::<Hotkey>);
    let selected_app = use_signal(|| None::<App>);
    use_effect(move || {
        if let (Some(hotkey), Some(app)) = (picked_hotkey(), selected_app()) {
            let action = Action::OpenApp(app);
            let _ = hotkey_service.write().bind_hotkey(hotkey, action);
        }
    });

    rsx! {
        div {
            class: "flex gap-4 p-6 items-center justify-center h-screen",
            style { "{include_str!(\"../../target/tailwind.css\")}" }
            HotkeyPicker { picked_hotkey }
            AppSelector { selected_app }
        }
    }
}
