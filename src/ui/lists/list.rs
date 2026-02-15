use std::collections::HashSet;
use std::hash::Hash;

use dioxus::prelude::*;
use dioxus_primitives::scroll_area::{ScrollArea, ScrollDirection};

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
            class: "flex flex-col flex-1 min-h-0 rounded-md",
            style: "background: var(--sidebar-background); color: var(--sidebar-foreground);",
            div {
                class: "flex items-center justify-between w-full !p-1",
                span {
                    class: "sidebar-group-label !pt-1.5",
                    "{title}"
                }
                ListMenu { selected }
            }
            div {
                class: "sidebar-group !p-1 flex-1 min-h-0",
                "data-sidebar": "group",
                div {
                    class: "sidebar-group-content flex-1 min-h-0",
                    "data-sidebar": "group-content",
                    ScrollArea {
                        direction: ScrollDirection::Vertical,
                        ul {
                            class: "sidebar-menu",
                            "data-sidebar": "menu",
                            for element in elements {
                                li {
                                    class: "sidebar-menu-item",
                                    "data-sidebar": "menu-item",
                                    ListRow { element, selected }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub(super) trait Renderable<I: Clone + Eq + Hash>: Identifiable<I> {
    fn render(&self) -> Element;
}
