use std::collections::HashSet;
use std::hash::Hash;

use dioxus::prelude::*;

use crate::ui::util::SmallButton;

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
            class: "flex items-center gap-1",
            SmallButton { onclick: add, "+" }
            SmallButton { onclick: remove, "-" }
        }
    }
}
