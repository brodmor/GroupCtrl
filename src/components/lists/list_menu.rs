use std::collections::HashSet;
use std::hash::Hash;

use dioxus::prelude::*;

#[derive(Clone)]
pub enum ListOperation<I>
where
    I: Clone + Eq + Hash + 'static,
{
    Add, // adding is interactive
    Remove(HashSet<I>),
}

#[component]
pub(super) fn ListMenu<I>(selected: Signal<HashSet<I>>) -> Element
where
    I: Clone + Eq + Hash + 'static,
{
    let sender = use_context::<UnboundedSender<ListOperation<I>>>();
    let my_sender = sender.clone();
    let add = move |_| {
        let _ = sender.unbounded_send(ListOperation::Add);
    };
    let remove = move |_| {
        let selection = selected().clone();
        selected.clear();
        let _ = my_sender.unbounded_send(ListOperation::Remove(selection));
    };

    rsx! {
        div {
            class: "flex",
            button {
                class: "btn btn-xs",
                onclick: add,
                "Add"
            }
            button {
                class: "btn btn-xs",
                disabled: selected().is_empty(),
                onclick: remove,
                "Remove"
            }
        }
    }
}
