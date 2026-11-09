//! Preserve the native macOS bundle icon and its appearance variants.
use std::path::Path;

fn in_app_bundle(exe: &Path) -> bool {
    let Some(macos) = exe.parent() else { return false };
    let Some(contents) = macos.parent() else { return false };
    let Some(bundle) = contents.parent() else { return false };
    macos.file_name().is_some_and(|n| n == "MacOS")
        && contents.file_name().is_some_and(|n| n == "Contents")
        && bundle.file_name().is_some_and(|n| {
            let name = n.to_string_lossy();
            name.len() > 4 && name.to_ascii_lowercase().ends_with(".app")
        })
}

pub(crate) fn viewport_icon(exe: Option<&Path>, png: &[u8]) -> egui::IconData {
    if cfg!(target_os = "macos") && exe.is_some_and(in_app_bundle) {
        // Some(empty), rather than no with_icon call, also blocks eframe's default icon.
        return egui::IconData::default();
    }
    eframe::icon_data::from_png_bytes(png).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundle_detection_requires_the_full_macos_layout() {
        assert!(in_app_bundle(Path::new("/Applications/Example.app/Contents/MacOS/Example")));
        assert!(in_app_bundle(Path::new("/Downloads/Example Beta.APP/Contents/MacOS/Example")));
        for path in ["target/release/example", "/opt/Contents/MacOS/example", "/.app/Contents/MacOS/example", "/Example.app/example"] {
            assert!(!in_app_bundle(Path::new(path)), "{path}");
        }
    }

    #[test]
    fn packaged_app_keeps_native_icon_but_unbundled_run_has_artwork() {
        let png = include_bytes!("../../../assets/app-icon/hicolor/256x256/apps/ai.storyteller.vectorcraft.png");
        let icon = viewport_icon(Some(Path::new("/Applications/Example.app/Contents/MacOS/Example")), png);
        if cfg!(target_os = "macos") {
            assert_eq!(icon, egui::IconData::default());
        } else {
            assert_eq!(icon.width, 256);
        }
        assert_eq!(viewport_icon(None, png).width, 256);
        assert_eq!(viewport_icon(Some(Path::new("target/release/example")), png).width, 256);
    }
}
