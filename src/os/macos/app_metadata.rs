use std::path::PathBuf;

use objc2::rc::Retained;
use objc2_app_kit::{NSBitmapImageFileType, NSBitmapImageRep, NSImage, NSWorkspace};
use objc2_foundation::{NSData, NSDictionary, NSFileManager, NSString};

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
        .and_then(|path| save_icon(path, bundle_id));
    App::new(bundle_id.to_string(), app_path, name, icon_path)
}

fn resolve_app_path(bundle_id: &str) -> Option<String> {
    let ns_id = NSString::from_str(bundle_id);
    let url = NSWorkspace::sharedWorkspace().URLForApplicationWithBundleIdentifier(&ns_id)?;
    Some(url.path()?.to_string())
}

fn resolve_name(app_path: &str) -> Option<String> {
    let name = NSFileManager::defaultManager()
        .displayNameAtPath(&NSString::from_str(app_path))
        .to_string();
    Some(name.strip_suffix(".app").unwrap_or(&name).to_string())
}

fn heuristic_name(bundle_id: &str) -> String {
    let name = bundle_id.split('.').next_back().unwrap_or(bundle_id);
    capitalize(name)
}

fn save_icon(app_path: &str, bundle_id: &str) -> Option<PathBuf> {
    let dir = crate::os::icons_dir();
    let png_path = dir.join(format!("{bundle_id}.png"));
    std::fs::create_dir_all(&dir).ok()?;
    let ns_path = NSString::from_str(app_path);
    let image = NSWorkspace::sharedWorkspace().iconForFile(&ns_path);
    let data = convert_icon(image)?;
    unsafe { std::fs::write(&png_path, data.as_bytes_unchecked()).ok()? };
    Some(png_path)
}

fn convert_icon(image: Retained<NSImage>) -> Option<Retained<NSData>> {
    let tiff = image.TIFFRepresentation()?;
    let reps = NSBitmapImageRep::imageRepsWithData(&tiff);
    // Find size closest to 128, preferably larger
    let rep = reps.into_iter().min_by_key(|rep| {
        let size = rep.pixelsWide().max(rep.pixelsHigh());
        if size >= 128 { size } else { isize::MAX - size }
    })?;
    unsafe {
        let rep: Retained<NSBitmapImageRep> = Retained::cast_unchecked(rep);
        rep.representationUsingType_properties(NSBitmapImageFileType::PNG, &NSDictionary::new())
    }
}
