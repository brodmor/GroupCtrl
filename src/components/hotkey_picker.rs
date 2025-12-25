use std::sync::Arc;

use dioxus::prelude::*;
use futures_util::StreamExt;

use crate::models::Hotkey;
use crate::services::{HotkeyCallback, SharedHotkeyCallback};
use crate::util::is_modifier;

#[component]
pub(super) fn HotkeyPicker(mut picked_hotkey: Signal<Option<Hotkey>>) -> Element {
    let mut recording = use_signal(|| false);
    let record_unregistered = move |evt: KeyboardEvent| {
        record_unregistered(recording, picked_hotkey, evt);
    };
    use_record_registered(recording, picked_hotkey);

    let label = if recording() {
        rsx! {
            span { style: "color: black", "Recording..." }
        }
    } else {
        match picked_hotkey() {
            None => rsx! {
                span { style: "color: gray", "None" }
            },
            Some(key) => rsx! {
                span { style: "color: black", "{key}" }
            },
        }
    };
    rsx! {
        div {
            onkeydown: record_unregistered, // globally registered keys never make it here
            tabindex: 0,
            button {
                onclick: move |_| recording.set(true),
                {label}
            }
        }
    }
}

fn record_unregistered(
    mut recording: Signal<bool>,
    mut picked_hotkey: Signal<Option<Hotkey>>,
    evt: KeyboardEvent,
) {
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
}

fn use_record_registered(mut recording: Signal<bool>, mut picked_hotkey: Signal<Option<Hotkey>>) {
    let listener = use_coroutine(move |mut receiver: UnboundedReceiver<Hotkey>| async move {
        while let Some(hotkey) = receiver.next().await {
            recording.set(false);
            picked_hotkey.set(Some(hotkey));
        }
    });
    let record_registered = use_context::<SharedHotkeyCallback>();
    use_effect(move || {
        record_registered.set(if !recording() {
            None
        } else {
            let listener_sender = listener.tx();
            Some(Arc::new(move |hotkey: Hotkey| {
                let _ = listener_sender.unbounded_send(hotkey);
            }) as HotkeyCallback)
        });
    });
}
