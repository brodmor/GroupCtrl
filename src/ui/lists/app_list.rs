use std::collections::HashSet;

use dioxus::prelude::*;

use super::list::{List, Renderable};
use crate::models::Identifiable;
use crate::os::App;

#[component]
pub fn AppList(apps: Vec<App>) -> Element {
    rsx! {
        div {
            class: "flex flex-col flex-1 min-h-0",
            List {
                title: "Apps".to_string(),
                elements: apps,
                selected: use_signal(HashSet::<String>::new),
            }
        }
    }
}

impl Renderable<String> for App {
    fn render(&self) -> Element {
        rsx! {
            span { "{self.id()}" }
        }
    }
}
