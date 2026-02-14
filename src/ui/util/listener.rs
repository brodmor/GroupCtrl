use dioxus::hooks::UnboundedSender;
use dioxus::prelude::{Callback, UnboundedReceiver, use_coroutine};
use futures_util::StreamExt;

pub fn use_listener<T: Clone + 'static>(callback: Callback<T>) -> UnboundedSender<T> {
    use_coroutine(move |mut receiver: UnboundedReceiver<T>| async move {
        while let Some(list_op) = receiver.next().await {
            callback.call(list_op);
        }
    })
    .tx()
}
