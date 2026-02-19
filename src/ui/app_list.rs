use std::collections::HashSet;

use dioxus::prelude::*;

use crate::models::Identifiable;
use crate::os::App;
use crate::ui::util::{AppLabel, ListMenu, use_selection};

#[component]
pub fn AppList(apps: Vec<App>) -> Element {
    let selected = use_signal(HashSet::<String>::new);
    rsx! {
        div {
            class: "flex flex-col flex-1 min-h-0 rounded-xl",
            style: "background: var(--sidebar-background); color: var(--sidebar-foreground);",
            div {
                class: "flex items-center justify-between w-full !p-2 !pb-0",
                label { r#for: "app-list", class: "pl-1.5", "Apps" }
                ListMenu { selected }
            }
            div {
                class: "sidebar-group !p-2 !pt-1 flex-1 min-h-0",
                "data-sidebar": "group",
                div {
                    class: "sidebar-group-content flex flex-col flex-1 min-h-0",
                    "data-sidebar": "group-content",
                    div {
                        class: "flex-1 min-h-0 overflow-y-auto",
                        ul {
                            id: "app-list",
                            class: "sidebar-menu",
                            "data-sidebar": "menu",
                            for app in apps {
                                li {
                                    class: "sidebar-menu-item",
                                    "data-sidebar": "menu-item",
                                    AppRow { app, selected }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AppRow(app: App, selected: Signal<HashSet<String>>) -> Element {
    let (is_selected, toggle) = use_selection(app.id(), selected);
    rsx! {
        button {
            class: "sidebar-menu-button",
            "data-sidebar": "menu-button",
            "data-size": "default",
            "data-active": is_selected(),
            onclick: move |e| toggle.call(e),
            AppLabel { app }
        }
    }
}
