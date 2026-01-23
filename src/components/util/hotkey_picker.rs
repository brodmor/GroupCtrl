use dioxus::prelude::*;
use futures_util::StreamExt;

use crate::models::Hotkey;
use crate::services::SharedSender;
use crate::util::is_modifier;

#[component]
pub fn HotkeyPicker(
    mut hotkey: Option<Hotkey>,
    set_hotkey: EventHandler<Option<Hotkey>>,
) -> Element {
    let mut recording = use_signal(|| false);
    use_record_registered(recording, set_hotkey);
    let onkeydown = move |evt: KeyboardEvent| record_unregistered(recording, set_hotkey, evt);

    let label = if recording() {
        rsx! {
            span { class: "text-base-content", "Recording..." }
        }
    } else {
        match hotkey {
            None => rsx! {
                span { class: "opacity-50", "None" }
            },
            Some(key) => rsx! {
                span { class: "text-base-content", "{key}" }
            },
        }
    };
    rsx! {
        div {
            role: "button",
            class: "btn btn-sm btn-outline w-fit outline-none",
            tabindex: 0,
            onkeydown, // globally registered keys never make it here
            onclick: move |_| recording.set(true),
            { label }
        }
    }
}

fn record_unregistered(
    mut recording: Signal<bool>,
    set_hotkey: EventHandler<Option<Hotkey>>,
    evt: KeyboardEvent,
) {
    let code = evt.code();
    if !recording() && code == Code::Enter {
        recording.set(true);
    }
    if !recording() || is_modifier(&code) {
        return;
    }

    set_hotkey.call(if code == Code::Escape {
        None
    } else {
        Some(Hotkey::new(evt.modifiers(), code))
    });
    recording.set(false);
}

fn use_record_registered(mut recording: Signal<bool>, set_hotkey: EventHandler<Option<Hotkey>>) {
    let listener = use_coroutine(move |mut receiver: UnboundedReceiver<Hotkey>| async move {
        while let Some(hotkey) = receiver.next().await {
            set_hotkey.call(Some(hotkey));
            recording.set(false);
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
