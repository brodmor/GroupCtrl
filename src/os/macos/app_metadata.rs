use std::path::PathBuf;

use objc2_app_kit::{NSBitmapImageFileType, NSBitmapImageRep, NSWorkspace};
use objc2_foundation::{NSDictionary, NSFileManager, NSString};

use super::app::App;
use crate::util::capitalize;

pub fn resolve(bundle_id: &str) -> App {
    let app_path = resolve_app_path(bundle_id);
    let name = app_path
        .as_deref()
        .and_then(resolve_name)
        .unwrap_or_else(|| heuristic_name(bundle_id));
    let icon_path = app_path
        .as_deref()
        .and_then(|path| convert_icon(path, bundle_id));
    App::new(bundle_id.to_string(), name, icon_path, app_path)
}

fn resolve_app_path(bundle_id: &str) -> Option<String> {
    let ns_id = NSString::from_str(bundle_id);
    let url = NSWorkspace::sharedWorkspace().URLForApplicationWithBundleIdentifier(&ns_id)?;
    Some(url.path()?.to_string())
}

fn resolve_name(app_path: &str) -> Option<String> {
    let ns_path = NSString::from_str(app_path);
    let name = NSFileManager::defaultManager().displayNameAtPath(&ns_path);
    Some(name.to_string())
}

fn heuristic_name(bundle_id: &str) -> String {
    let name = bundle_id.split('.').next_back().unwrap_or(bundle_id);
    capitalize(name)
}

fn convert_icon(app_path: &str, bundle_id: &str) -> Option<PathBuf> {
    let dir = crate::os::icons_dir();
    let png_path = dir.join(format!("{bundle_id}.png"));
    // if png_path.metadata().is_ok() {
    //     return Some(png_path);
    // }
    std::fs::create_dir_all(&dir).ok()?;
    let ns_path = NSString::from_str(app_path);
    let image = NSWorkspace::sharedWorkspace().iconForFile(&ns_path);
    let tiff_data = image.TIFFRepresentation()?;
    let rep = NSBitmapImageRep::imageRepWithData(&tiff_data)?;
    let png_data = unsafe {
        rep.representationUsingType_properties(NSBitmapImageFileType::PNG, &NSDictionary::new())
    }?;
    let bytes = unsafe { png_data.as_bytes_unchecked() };
    std::fs::write(&png_path, bytes).ok()?;
    Some(png_path)
}
