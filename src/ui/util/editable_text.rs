use std::rc::Rc;

use dioxus::prelude::*;

use crate::components::input::Input;
use crate::os::{EditActions, System};

#[derive(PartialEq, Clone, Copy)]
pub enum InputMode {
    Edit,
    Create { on_cancel: Callback<()> },
}

#[component]
pub fn EditableText(
    text: Signal<String>,
    placeholder: String,
    starting_mode: InputMode,
) -> Element {
    let mut draft = use_signal(|| match starting_mode {
        InputMode::Edit => text(),
        InputMode::Create { .. } => String::new(),
    });
    let mut input_handle = use_signal(|| None::<Rc<MountedData>>);
    let set_focus = move |focus: bool| {
        if let Some(handle) = input_handle() {
            spawn(async move { drop(handle.set_focus(focus).await) });
        }
    };
    use_effect(move || {
        if let InputMode::Create { .. } = starting_mode {
            set_focus(true);
        }
    });

    let mut mode = use_signal(|| starting_mode);
    let mut cancel = move || match mode() {
        InputMode::Edit => draft.set(text()),
        InputMode::Create { on_cancel } => on_cancel.call(()),
    };
    let onkeydown = move |evt: KeyboardEvent| match evt.key() {
        Key::Enter => {
            mode.set(InputMode::Edit);
            if draft().trim().is_empty() {
                cancel();
            } else {
                text.set(draft());
            }
            set_focus(false);
        }
        Key::Escape => {
            cancel();
            set_focus(false);
        }
        #[cfg(target_os = "macos")]
        Key::Character(c) if c == "a" && evt.modifiers().contains(Modifiers::META) => {
            System::select_all();
        }
        _ => (),
    };
    let onblur = move |_| cancel();

    rsx! {
        Input {
            class: "input text-sm",
            value: "{draft}",
            placeholder: "{placeholder}",
            onmounted: move |evt: MountedEvent| input_handle.set(Some(evt.data())),
            oninput: move |evt: FormEvent| draft.set(evt.value()),
            onkeydown,
            onblur,
        }
    }
}
