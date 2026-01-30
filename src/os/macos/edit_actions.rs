use objc2::sel;
use objc2_app_kit::NSApplication;
use objc2_foundation::MainThreadMarker;

use crate::os::macos::System;

pub trait EditActions {
    fn select_all();
}

impl EditActions for System {
    fn select_all() {
        if let Some(mtm) = MainThreadMarker::new() {
            let app = NSApplication::sharedApplication(mtm);
            unsafe {
                app.sendAction_to_from(sel!(selectAll:), None, None);
            }
        } else {
            log::warn!("Could not get MainThreadMarker for Select All action");
        }
    }
}
