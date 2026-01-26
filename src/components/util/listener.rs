use dioxus::prelude::{EventHandler, UnboundedReceiver, use_coroutine};
use futures_util::StreamExt;

pub fn spawn_listener<T: Clone + 'static>(callback: EventHandler<T>) {
    use_coroutine(move |mut receiver: UnboundedReceiver<T>| async move {
        while let Some(list_op) = receiver.next().await {
            callback.call(list_op);
        }
    });
}
