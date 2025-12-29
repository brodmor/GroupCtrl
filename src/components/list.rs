use std::collections::HashSet;
use std::hash::Hash;

use dioxus::prelude::*;

use crate::components::list_cell::ListCell;
use crate::os::{Keyboard, KeyboardBehavior};

#[component]
pub fn List<E, I>(elements: Vec<E>, selected: Signal<HashSet<I>>) -> Element
where
    I: Clone + Eq + Hash + 'static,
    E: ListCell<I> + Clone + PartialEq + 'static,
{
    rsx! {
        ListMenu { selected }
        for element in elements {
            Cell { element, selected }
        }
    }
}

#[component]
fn ListMenu<I>(selected: Signal<HashSet<I>>) -> Element
where
    I: Clone + Eq + Hash + 'static,
{
    let sender = use_context::<UnboundedSender<CellChange<I>>>();
    let my_sender = sender.clone();
    let add = move |_| {
        let _ = sender.unbounded_send(CellChange::Add);
    };
    let remove = move |_| {
        let selection = selected.read().clone();
        selected.clear();
        let _ = my_sender.unbounded_send(CellChange::Remove(selection));
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
                onclick: remove,
                "Remove"
            }

        }
    }
}

#[component]
fn Cell<E, I>(element: E, mut selected: Signal<HashSet<I>>) -> Element
where
    I: Clone + Eq + Hash + 'static,
    E: ListCell<I> + Clone + PartialEq + 'static,
{
    let element_id = element.id();
    let is_selected = selected.read().contains(&element_id);
    let toggle_active = move |evt: Event<MouseData>| {
        let mut sel = selected.write();
        if Keyboard::is_multi_select(evt.modifiers()) {
            if !sel.contains(&element_id) {
                sel.insert(element_id.clone());
            } else {
                sel.remove(&element_id);
            }
        } else {
            sel.clear();
            sel.insert(element_id.clone());
        }
    };
    rsx! {
        button {
            // TODO maybe daisy join buttons for styling
           class: format_args!(
                "btn btn-sm w-full justify-start {}",
                if is_selected { "btn-active" } else { "btn-ghost" }
            ),
            onclick: toggle_active,
            { element.render() }
        }
    }
}

#[derive(Clone)]
pub enum CellChange<I>
where
    I: Clone + Eq + Hash + 'static,
{
    Add, // adding is interactive
    Remove(HashSet<I>),
}
