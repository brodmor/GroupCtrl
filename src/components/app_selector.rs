use dioxus::prelude::*;

use crate::os::{App, AppDialog, AppSelection};

#[component]
pub(super) fn AppSelector(mut selected_app: Signal<Option<App>>) -> Element {
    let pick_app = move |_| {
        spawn(async move {
            if let Ok(Some(app)) = AppDialog::select_app().await {
                selected_app.set(Some(app));
            }
        });
    };

    let app_display = match selected_app() {
        Some(app) => app.to_string(),
        None => "No app selected".to_string(),
    };

    rsx! {
        div {
            class: "flex gap-2 items-center",
            span { "{app_display}" }
            button {
                onclick: pick_app,
                "Pick App"
            }
        }
    }
}
