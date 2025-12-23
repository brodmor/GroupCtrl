use super::app::App;
use crate::os::prelude::Openable;

use anyhow::bail;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
mod win32 {
    pub use windows::Win32::Foundation::*;
    pub use windows::Win32::System::Threading::*;
    pub use windows::Win32::UI::WindowsAndMessaging::*;
}

// distinction between launch and focus? Could keep Openable for interface name
impl Openable for App {
    fn open(&self) -> anyhow::Result<()> {
        let main_windows = collect_main_windows()?;
        if let Some(window) = find_matching_window(&main_windows, self.exe_path.as_str()) {
            focus_window(window)?;
            println!("Activated window");
        } else {
            println!("No window found");
        }
        Ok(())
    }
}

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

fn collect_main_windows() -> windows::core::Result<Vec<win32::HWND>> {
    extern "system" fn collect_window_callback(
        window: win32::HWND,
        lparam: win32::LPARAM,
    ) -> win32::BOOL {
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
    windows.retain(|&hwnd| is_main_window(hwnd));
    Ok(windows)
}

fn find_matching_window(windows: &[win32::HWND], target_exe: &str) -> Option<win32::HWND> {
    for &window in windows {
        let mut process_id = 0u32;
        unsafe {
            // kernel lookup, inexpensive
            win32::GetWindowThreadProcessId(window, Some(&mut process_id));
        }
        if let Ok(exe) = get_process_exe_path(process_id)
            && exe.to_lowercase() == target_exe.to_lowercase()
        {
            return Some(window);
        }
    }
    None
}

/// Somewhat expensive -> cache
fn get_process_exe_path(process_id: u32) -> windows::core::Result<String> {
    unsafe {
        let handle =
            win32::OpenProcess(win32::PROCESS_QUERY_LIMITED_INFORMATION, false, process_id)?;
        let mut buffer = [0u16; 1024];
        let mut size = buffer.len() as u32;
        let pwstr = windows::core::PWSTR(buffer.as_mut_ptr());
        win32::QueryFullProcessImageNameW(handle, win32::PROCESS_NAME_WIN32, pwstr, &mut size)?;
        win32::CloseHandle(handle)?;
        Ok(OsString::from_wide(&buffer[..size as usize])
            .to_string_lossy()
            .into_owned())
    }
}

fn focus_window(window: win32::HWND) -> anyhow::Result<()> {
    unsafe {
        if win32::IsIconic(window).as_bool() {
            // undo minimization
            if !win32::ShowWindow(window, win32::SW_RESTORE).as_bool() {
                bail!("syscall 'ShowWindow' failed")
            }
        }
        if !win32::SetForegroundWindow(window).as_bool() {
            bail!("syscall 'SetForegroundWindow' failed")
        }
    }
    Ok(())
}
