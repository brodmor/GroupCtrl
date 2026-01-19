use dioxus::prelude::*;
use futures_util::StreamExt;

use crate::models::Hotkey;
use crate::services::SharedSender;
use crate::util::is_modifier;

#[component]
pub fn HotkeyPicker(mut picked_hotkey: Signal<Option<Hotkey>>) -> Element {
    let mut recording = use_signal(|| false);
    let record_unregistered = move |evt: KeyboardEvent| {
        record_unregistered(recording, picked_hotkey, evt);
    };
    use_record_registered(recording, picked_hotkey);

    let label = if recording() {
        rsx! {
            span { class: "text-base-content", "Recording..." }
        }
    } else {
        match picked_hotkey() {
            None => rsx! {
                span { class: "opacity-50", "None" }
            },
            Some(key) => rsx! {
                // TODO use DaisyUI kbd class
                span { class: "text-base-content", "{key}" }
            },
        }
    };
    rsx! {
        div {
            role: "button",
            class: "btn btn-sm btn-outline w-fit outline-none",
            tabindex: 0,
            onkeydown: record_unregistered, // globally registered keys never make it here
            onclick: move |_| recording.set(true),
            { label }
        }
    }
}

fn record_unregistered(
    mut recording: Signal<bool>,
    mut picked_hotkey: Signal<Option<Hotkey>>,
    evt: KeyboardEvent,
) {
    let code = evt.code();
    if !recording() && code == Code::Enter {
        recording.set(true);
    }
    if !recording() || is_modifier(&code) {
        return;
    }

    recording.set(false);
    picked_hotkey.set(if code == Code::Escape {
        None
    } else {
        Some(Hotkey::new(evt.modifiers(), code))
    })
}

fn use_record_registered(mut recording: Signal<bool>, mut picked_hotkey: Signal<Option<Hotkey>>) {
    let listener = use_coroutine(move |mut receiver: UnboundedReceiver<Hotkey>| async move {
        while let Some(hotkey) = receiver.next().await {
            recording.set(false);
            picked_hotkey.set(Some(hotkey));
        }
    });
    let record_registered_sender = use_context::<SharedSender<Hotkey>>();
    use_effect(move || {
        record_registered_sender.set(if !recording() {
            None
        } else {
            Some(listener.tx())
        });
    });
}
