use dioxus::prelude::*;
use global_hotkey::hotkey::Code;

use crate::models::hotkey::Hotkey;
use crate::util::convert::convert_hotkey;

#[component]
pub fn HotkeyPicker(mut picked_hotkey: Signal<Option<Hotkey>>) -> Element {
    let mut recording = use_context::<Signal<bool>>();

    let start_recording = move |_| {
        recording.set(true);
    };

    let handle_keydown = move |evt: KeyboardEvent| {
        if !recording() {
            return;
        }

        if let Some(hotkey) = convert_hotkey(&evt) {
            // Only stop recording when we successfully capture a non-modifier key
            recording.set(false);
            log::info!("Captured hotkey: {}", hotkey);

            // Escape clears the hotkey
            if hotkey.0.key == Code::Escape {
                picked_hotkey.set(None);
            } else {
                picked_hotkey.set(Some(hotkey));
            }
        } else {
            // Modifier-only keys are filtered - keep recording
            log::debug!("Ignoring modifier key: {}", evt.code());
        }
    };

    let label = if recording() {
        "Recording...".to_string()
    } else {
        match picked_hotkey() {
            None => "None".to_string(),
            Some(key) => key.to_string(),
        }
    };

    let label_color = if picked_hotkey().is_none() {
        "#888888"
    } else {
        "black"
    };

    rsx! {
        div {
            onkeydown: handle_keydown,
            tabindex: 0,
            button {
                onclick: start_recording,
                style: "color: {label_color};",
                "{label}"
            }
        }
    }
}
