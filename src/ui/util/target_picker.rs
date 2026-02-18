use dioxus::prelude::*;

use super::AppLabel;
use crate::components::select::*;
use crate::os::{App, AppMetadata};

#[component]
pub fn TargetPicker(
    apps: Vec<App>,
    target: Option<App>,
    set_target: Callback<Option<App>>,
) -> Element {
    let value: Option<Option<Option<App>>> = Some(Some(target.clone()));
    rsx! {
        div { class: "flex-1",
        Select::<Option<App>> {
            value,
            on_value_change: move |choice: Option<Option<App>>| {
                set_target.call(choice.flatten());
            },
            SelectTrigger {
                match &target {
                    Some(app) => rsx! { AppLabel { app: app.clone() } },
                    None => rsx! { span { class: "text-(--muted-text)", "Most Recent" } },
                }
            }
            SelectList {
                SelectOption::<Option<App>> {
                    value: None::<App>,
                    text_value: "Most Recent".to_string(),
                    index: 0usize,
                    "Most Recent"
                }
                for (i, app) in apps.iter().enumerate() {
                    SelectOption::<Option<App>> {
                        value: Some(app.clone()),
                        text_value: app.name().to_string(),
                        index: i + 1,
                        AppLabel { app: app.clone() }
                    }
                }
            }
        }
        }
    }
}
