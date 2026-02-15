use std::collections::HashSet;
use std::hash::Hash;

use dioxus::prelude::*;

use crate::os::{KeyboardBehavior, System};

pub fn use_selection<I>(
    id: I,
    mut selected: Signal<HashSet<I>>,
) -> (Memo<bool>, EventHandler<MouseEvent>)
where
    I: Clone + Eq + Hash + 'static,
{
    let id_for_memo = id.clone();
    let is_selected = use_memo(move || selected().contains(&id_for_memo));
    let toggle = move |e: MouseEvent| {
        let mut sel = selected.write();
        if System::is_multi_select(e.modifiers()) {
            if sel.contains(&id) {
                sel.remove(&id);
            } else {
                sel.insert(id.clone());
            }
        } else {
            sel.clear();
            sel.insert(id.clone());
        }
    };

    (is_selected, EventHandler::new(toggle))
}
