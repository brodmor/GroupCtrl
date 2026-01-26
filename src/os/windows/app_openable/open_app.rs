use anyhow::bail;

use super::win32::focus;

pub fn open_app(exe_path: &str) -> anyhow::Result<()> {
    if !focus(exe_path)? {
        bail!("Couldn't find window");
    };
    Ok(())
}
