use std::collections::HashSet;
use std::hash::Hash;

use dioxus::prelude::*;
use lucide_dioxus::{Minus, Plus};

use crate::components::button::{Button, ButtonVariant};

#[derive(Clone)]
pub enum ListOperation<I>
where
    I: Clone + Eq + Hash + 'static,
{
    Add, // adding is interactive
    Remove(I),
}

#[component]
pub fn ListMenu<I>(selected: Signal<HashSet<I>>) -> Element
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
            SmallButton { onclick: add, disabled: false, Plus { stroke_width: 4 } }
            SmallButton { onclick: remove, disabled: selected().is_empty(), Minus { stroke_width: 4 } }
        }
    }
}

#[component]
fn SmallButton(onclick: EventHandler<MouseEvent>, disabled: bool, children: Element) -> Element {
    rsx! {
        Button {
            variant: ButtonVariant::Secondary,
            class: "!p-0 size-6 grid place-items-center [&>svg]:size-3",
            onclick,
            disabled,
            {children}
        }
    }
}
