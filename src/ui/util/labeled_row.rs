use dioxus::prelude::*;

#[component]
pub fn LabeledRow(label: String, children: Element) -> Element {
    rsx! {
        div {
            class: "flex items-center btn-wide",
            span { class: "text-sm w-10", "{label}" }
            { children }
        }
    }
}
