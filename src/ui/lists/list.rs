use std::collections::HashSet;
use std::hash::Hash;

use dioxus::prelude::*;

use crate::models::Identifiable;
use crate::ui::lists::list_menu::ListMenu;
use crate::ui::lists::list_row::ListRow;

#[component]
pub(super) fn List<E, I>(title: String, elements: Vec<E>, selected: Signal<HashSet<I>>) -> Element
where
    I: Clone + Eq + Hash + 'static,
    E: Renderable<I> + Clone + PartialEq + 'static,
{
    rsx! {
        div {
            class: "flex justify-between mb-1",
            span { class: "font-bold text-md ml-1", "{title}" }
            ListMenu { selected }
        }
        for element in elements {
            ListRow { element, selected }
        }
    }
}

pub(super) trait Renderable<I: Clone + Eq + Hash>: Identifiable<I> {
    fn render(&self) -> Element;
}
