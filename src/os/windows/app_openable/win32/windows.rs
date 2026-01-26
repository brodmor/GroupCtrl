use super::api as win32;
use super::pid_to_exe::pid_to_exe;

fn is_main_window(window: win32::HWND) -> bool {
    fn is_tool_window(window: win32::HWND) -> bool {
        let ex_style = unsafe { win32::GetWindowLongW(window, win32::GWL_EXSTYLE) };
        ex_style as u32 & win32::WS_EX_TOOLWINDOW.0 != 0
    }
    unsafe {
        win32::IsWindowVisible(window).as_bool() // is visible
            && win32::GetWindowTextLengthW(window) > 0 // has title
            && win32::GetParent(window).map_or(true, |p| p.is_invalid()) // does not have parent
            && !is_tool_window(window)
    }
}

pub(super) fn collect_main_windows() -> windows::core::Result<Vec<win32::HWND>> {
    extern "system" fn collect_window_callback(
        window: win32::HWND,
        lparam: win32::LPARAM,
    ) -> windows::core::BOOL {
        unsafe {
            let windows = &mut *(lparam.0 as *mut Vec<win32::HWND>);
            windows.push(window);
            true.into() // continue enumeration
        }
    }

    let mut windows = Vec::new();
    let lparam = win32::LPARAM(&mut windows as *mut _ as isize);
    unsafe {
        win32::EnumWindows(Some(collect_window_callback), lparam)?;
    }
    windows.retain(|&window| is_main_window(window));
    Ok(windows)
}

pub(super) fn find_matching_window(
    windows: &[win32::HWND],
    target_exe: &str,
) -> windows::core::Result<Option<win32::HWND>> {
    for &window in windows {
        let mut process_id = 0u32;
        unsafe {
            // kernel lookup, inexpensive
            win32::GetWindowThreadProcessId(window, Some(&mut process_id));
        }
        if pid_to_exe(process_id)?.to_lowercase() == target_exe.to_lowercase() {
            return Ok(Some(window));
        }
    }
    Ok(None)
}
