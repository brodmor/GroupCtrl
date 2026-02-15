use dioxus::prelude::*;

use crate::components::button::{Button, ButtonVariant};

#[component]
pub fn SmallButton(onclick: EventHandler<MouseEvent>, children: Element) -> Element {
    rsx! {
        Button {
            variant: ButtonVariant::Secondary,
            class: "!p-0 size-6 flex items-center justify-center",
            onclick: move |e| onclick.call(e),
            {children}
        }
    }
}
