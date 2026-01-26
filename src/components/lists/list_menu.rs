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
    let tx = use_coroutine_handle::<ListOperation<I>>();
    let add = move |_| tx.send(ListOperation::Add);
    let remove = move |_| tx.send(ListOperation::Remove(selected()));

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
