use dioxus::prelude::*;

use crate::models::Identifiable;
use crate::os::App;

#[component]
pub fn MainAppPicker(
    apps: Vec<App>,
    main_app: Option<App>,
    set_main_app: Callback<Option<App>>,
) -> Element {
    let onchange_apps = apps.clone();
    let onchange = move |evt: Event<FormData>| {
        let value = evt.value();
        let app = onchange_apps.iter().find(|app| app.id() == value).cloned();
        set_main_app.call(app);
    };
    let placeholder_class = if main_app.is_none() { "opacity-50" } else { "" };
    rsx! {
        select {
            class: "btn btn-sm btn-outline flex-1 focus:outline-none {placeholder_class}",
            value: main_app.map(|a| a.id()).unwrap_or_default(),
            onchange,
            option { value: "", class: "opacity-50", "(Most Recent)" }
            for app in apps.iter() {
                option {
                    value: app.id(),
                    "{app}"
                }
            }
        }
    }
}
