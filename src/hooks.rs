use std::sync::Arc;

use dioxus::prelude::*;
use futures_util::stream::StreamExt;

use crate::models::Hotkey;
use crate::services::{RecordRegistered, RecordRegisteredFn};

/// Handles hotkeys that are already globally registered
/// Nice side effect: Collisions can only occur in this scenario
pub fn use_record_registered(
    mut recording: Signal<bool>,
    mut picked_hotkey: Signal<Option<Hotkey>>,
) {
    let record_registered = use_context::<RecordRegistered>();
    let hotkey_coroutine = use_coroutine(move |mut rx: UnboundedReceiver<Hotkey>| async move {
        while let Some(hotkey) = rx.next().await {
            recording.set(false);
            picked_hotkey.set(Some(hotkey));
        }
    });
    // This is called by the OS thread and therefore can't manipulate UI
    // Thus we need to send UI updates to a coroutine
    use_effect(move || {
        let callback = if recording() {
            let tx = hotkey_coroutine.tx();
            let cb: RecordRegisteredFn = Arc::new(move |hotkey: Hotkey| {
                let _ = tx.unbounded_send(hotkey);
            });
            Some(cb)
        } else {
            None
        };
        record_registered.set(callback);
    });
}
