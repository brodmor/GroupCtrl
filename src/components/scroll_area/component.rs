use dioxus::prelude::*;
use dioxus_primitives::scroll_area::{self, ScrollAreaProps};

#[component]
pub fn ScrollArea(props: ScrollAreaProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        { scroll_area::ScrollArea(props) }
    }
}
