use dioxus::prelude::{EventHandler, UnboundedReceiver, UnboundedSender, use_coroutine};
use futures_util::StreamExt;

pub fn spawn_listener<T>(callback: EventHandler<T>) -> UnboundedSender<T>
where
    T: Clone + 'static,
{
    use_coroutine(move |mut receiver: UnboundedReceiver<T>| async move {
        while let Some(list_op) = receiver.next().await {
            callback.call(list_op);
        }
    })
    .tx()
}
