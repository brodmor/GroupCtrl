use std::collections::HashMap;

use dioxus::desktop::window;
use dioxus::prelude::*;
use global_hotkey::HotKeyState;
use log::error;

use crate::components::app_selector::AppSelector;
use crate::components::hotkey_picker::HotkeyPicker;
use crate::models::action::Action;
use crate::models::hotkey::Hotkey;
use crate::os::App;
use crate::services::hotkey::HotkeyService;

#[component]
pub fn Root() -> Element {
    let selected_app = use_signal(|| None::<App>);
    let picked_hotkey = use_signal(|| None::<Hotkey>);
    // has to be global because recording is global state so we can pause all hotkeys
    // should be moved to hotkey service though because that's where it ought to be used
    let recording = use_signal(|| false);

    // Provide recording state to child components
    use_context_provider(|| recording);

    use_effect(move || {
        if let (Some(app), Some(hotkey)) = (selected_app(), picked_hotkey()) {
            let action = Action::OpenApp(app);

            // TODO go through hotkey service instead
            let _ = window().create_shortcut(hotkey.0, move |state| {
                // Only trigger on key press, not release
                if state == HotKeyState::Pressed && !recording() {
                    let _ = action.execute();
                }
            });
        }
    });

    rsx! {
        div {
            style: "display: flex; gap: 10px; padding: 20px;",
            HotkeyPicker { picked_hotkey }
            AppSelector { selected_app }
        }
    }
}
