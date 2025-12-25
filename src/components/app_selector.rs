use dioxus::prelude::*;

use crate::os::prelude::AppPickerTrait;
use crate::os::{App, AppPicker};

#[component]
pub fn AppSelector(mut selected_app: Signal<Option<App>>) -> Element {
    let pick_app = move |_| {
        spawn(async move {
            if let Ok(Some(app)) = AppPicker::pick_app().await {
                selected_app.set(Some(app));
            }
        });
    };

    let app_display = match selected_app() {
        Some(app) => app.bundle_id,
        None => "No app selected".to_string(),
    };

    rsx! {
        div {
            style: "display: flex; gap: 10px; align-items: center;",
            span { "{app_display}" }
            button {
                onclick: pick_app,
                "Pick App"
            }
        }
    }
}
