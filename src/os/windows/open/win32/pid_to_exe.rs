use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::sync::{LazyLock, Mutex};

use super::api as win32;

static EXE_CACHE: LazyLock<Mutex<HashMap<u32, String>>> = LazyLock::new(Default::default);

pub(super) fn pid_to_exe(process_id: u32) -> windows::core::Result<String> {
    Ok(match EXE_CACHE.lock().unwrap().entry(process_id) {
        Occupied(e) => e.into_mut().clone(),
        Vacant(e) => e.insert(lookup_process_exe_path(process_id)?).clone(),
    })
}

fn lookup_process_exe_path(process_id: u32) -> windows::core::Result<String> {
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
