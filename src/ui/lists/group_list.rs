use std::collections::HashSet;

use dioxus::prelude::*;
use uuid::Uuid;

use super::list::{List, Renderable};
use crate::models::Group;

#[component]
pub fn GroupList(groups: Vec<Group>, selected: Signal<HashSet<Uuid>>) -> Element {
    rsx! {
        List {
            title: "Groups".to_string(),
            elements: groups,
            selected,
        }
    }
}

impl Renderable<Uuid> for Group {
    fn render(&self) -> Element {
        rsx! {
            span { "{self.name}" }
        }
    }
}
