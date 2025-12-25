use dioxus::prelude::*;

use crate::hooks::use_record_registered;
use crate::models::Hotkey;

fn is_modifier(code: &Code) -> bool {
    let code_str = code.to_string();
    code_str.contains("Control")
        || code_str.contains("Meta")
        || code_str.contains("Alt")
        || code_str.contains("Shift")
}

#[component]
pub(super) fn HotkeyPicker(mut picked_hotkey: Signal<Option<Hotkey>>) -> Element {
    let mut recording = use_signal(|| false);
    use_record_registered(recording, picked_hotkey);

    let start_recording = move |_| {
        recording.set(true);
    };

    let handle_keydown = move |evt: KeyboardEvent| {
        let code = evt.code();
        if !recording() || is_modifier(&code) {
            return;
        }
        recording.set(false);
        picked_hotkey.set(if code == Code::Escape {
            None
        } else {
            Some(Hotkey::new(evt.modifiers(), code))
        })
    };

    let label = if recording() {
        "Recording...".to_string()
    } else {
        match picked_hotkey() {
            None => "None".to_string(),
            Some(key) => key.to_string(),
        }
    };
    let label_color = if label == "None" { "gray" } else { "black" };
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
