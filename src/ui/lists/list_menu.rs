use std::collections::HashSet;
use std::hash::Hash;

use dioxus::prelude::*;

#[derive(Clone)]
pub enum ListOperation<I>
where
    I: Clone + Eq + Hash + 'static,
{
    Add, // adding is interactive
    Remove(I),
}

#[component]
pub(super) fn ListMenu<I>(selected: Signal<HashSet<I>>) -> Element
where
    I: Clone + Eq + Hash + 'static,
{
    let tx = use_coroutine_handle::<ListOperation<I>>();
    let add = move |_| tx.send(ListOperation::Add);
    let remove = move |_| {
        for item in selected() {
            tx.send(ListOperation::Remove(item))
        }
    };

    rsx! {
        div {
            class: "flex gap-1",
            button {
                class: "btn btn-xs btn-square",
                onclick: add,
                "+"
            }
            button {
                class: "btn btn-xs btn-square",
                disabled: selected().is_empty(),
                onclick: remove,
                "-"
            }
        }
    }
}
