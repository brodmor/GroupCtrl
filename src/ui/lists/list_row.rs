use std::collections::HashSet;
use std::hash::Hash;

use dioxus::prelude::*;

use super::list::Renderable;
use crate::ui::util::use_selection;

#[component]
pub(super) fn ListRow<E, I>(element: E, selected: Signal<HashSet<I>>) -> Element
where
    I: Clone + Eq + Hash + 'static,
    E: Renderable<I> + Clone + PartialEq + 'static,
{
    let (is_selected, toggle) = use_selection(element.id(), selected);

    rsx! {
        button {
            class: "sidebar-menu-button",
            "data-sidebar": "menu-button",
            "data-size": "default",
            "data-active": is_selected(),
            onclick: move |e| toggle.call(e),
            { element.render() }
        }
    }
}
