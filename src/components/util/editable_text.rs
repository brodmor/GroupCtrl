use std::rc::Rc;

use dioxus::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum InputMode {
    Edit,
    Create { on_cancel: EventHandler<()> },
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
            drop(handle.set_focus(focus));
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
        _ => (),
    };
    let onblur = move |_| cancel();

    rsx! {
        input {
            class: "input input-ghost input-xs font-bold text-sm w-full p-1",
            value: "{draft}",
            placeholder: "{placeholder}",
            onmounted: move |evt| input_handle.set(Some(evt.data())),
            oninput: move |evt| draft.set(evt.value()),
            onblur,
            onkeydown,
        }
    }
}
