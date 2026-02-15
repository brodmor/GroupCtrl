use std::collections::HashSet;
use std::sync::{Arc, RwLock};

use dioxus::desktop::window;
use dioxus::prelude::*;
use uuid::Uuid;

use crate::components::sidebar::*;
use crate::models::{Config, Hotkey, Identifiable};
use crate::services::{ActionService, ConfigReader, ConfigService};
use crate::ui::group_config::GroupConfig;
use crate::ui::lists::ListOperation;
use crate::ui::util::{SmallButton, use_listener};

#[component]
pub fn Root() -> Element {
    use_effect(move || window().set_decorations(true));

    let config_service = use_config_service();
    let selected = use_signal(HashSet::<Uuid>::new);
    let in_creation_group = use_signal(|| None::<Uuid>);
    use_group_list_listener(config_service, selected, in_creation_group);

    let active_group = use_memo(move || {
        if selected().len() == 1 {
            selected().iter().next().copied()
        } else {
            None
        }
    });

    let tx = use_coroutine_handle::<ListOperation<Uuid>>();

    let add = move |_: MouseEvent| tx.send(ListOperation::Add);
    let remove = move |_: MouseEvent| {
        for item in selected() {
            tx.send(ListOperation::Remove(item));
        }
    };

    let groups = config_service.read().config().groups().clone();

    rsx! {
        div {
            "data-theme": "dim",
            SidebarProvider {
                Sidebar {
                    side: SidebarSide::Left,
                    variant: SidebarVariant::Sidebar,
                    collapsible: SidebarCollapsible::None,

                    SidebarHeader {
                        class: "!p-1",
                        div {
                            class: "flex items-center justify-between w-full",
                            span {
                                class: "sidebar-group-label !pt-1.5",
                                "Groups"
                            }
                            div {
                                class: "flex items-center gap-1",
                                SmallButton { onclick: add, "+" }
                                SmallButton { onclick: remove, "-" }
                            }
                        }
                    }
                    SidebarContent {
                        SidebarGroup { class: "!p-1",
                            SidebarGroupContent {
                                SidebarMenu {
                                    for group in groups {
                                        GroupMenuItem {
                                            key: "{group.id()}",
                                            group_id: group.id(),
                                            name: group.name.clone(),
                                            selected,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                SidebarInset {
                    if let Some(group_id) = active_group() {
                        GroupConfig {
                            key: "{group_id}",
                            config_service,
                            group_id,
                            in_creation_group
                        }
                    }
                }
            }
        }
    }
}

fn use_config_service() -> Signal<ConfigService> {
    let config = Arc::new(RwLock::new(Config::load().unwrap_or_default()));
    let config_reader = ConfigReader::new(config.clone());
    let action_service = ActionService::new(config_reader);

    let active_recorder = use_context_provider(|| Signal::new(None::<UnboundedSender<Hotkey>>));
    let hotkey_sender = use_listener(Callback::new(move |(hotkey, action)| {
        if let Some(sender) = active_recorder() {
            sender.unbounded_send(hotkey).unwrap();
        } else {
            let service = action_service.clone();
            spawn(async move {
                service.execute(&action).await;
            });
        }
    }));

    use_signal(|| ConfigService::new(config, hotkey_sender))
}

#[component]
fn GroupMenuItem(group_id: Uuid, name: String, selected: Signal<HashSet<Uuid>>) -> Element {
    let is_active = use_memo(move || selected().contains(&group_id));

    rsx! {
        SidebarMenuItem {
            SidebarMenuButton {
                is_active: is_active(),
                onclick: move |e: MouseEvent| {
                    if e.modifiers().meta() || e.modifiers().ctrl() {
                        if selected().contains(&group_id) {
                            selected.write().remove(&group_id);
                        } else {
                            selected.write().insert(group_id);
                        }
                    } else {
                        selected.write().clear();
                        selected.write().insert(group_id);
                    }
                },
                span { "{name}" }
            }
        }
    }
}

fn use_group_list_listener(
    mut config_service: Signal<ConfigService>,
    mut selected: Signal<HashSet<Uuid>>,
    mut in_creation_group: Signal<Option<Uuid>>,
) {
    use_listener(Callback::new(move |list_operation: ListOperation<Uuid>| {
        selected.write().clear();
        match list_operation {
            ListOperation::Add => {
                let group_id = config_service.write().add_group("New Group".to_string());
                selected.write().insert(group_id);
                in_creation_group.set(Some(group_id));
            }
            ListOperation::Remove(group_id) => {
                config_service.write().remove_group(group_id);
            }
        }
    }));
}
