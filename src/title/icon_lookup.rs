use freedesktop_desktop_entry::{default_paths, DesktopEntry};
use std::fs;
use std::path::PathBuf;

const ICON_SIZE: u16 = 24;
const DEFAULT_ICON: &str = "/home/latipun/.config/eww/assets/hyprland.png";
const ICON_THEME: &str = "Papirus-Dark";

/// Guess the icon path of the given app class (a.k.a. app_id or window class), or `DEFAULT_ICON`
/// if no suitable icon is found.
pub fn guess_icon(app_class: &str) -> PathBuf {
    if let Some(desktop_file) = guess_desktop_file(app_class) {
        if let Some(icon_name) = get_icon_from_desktop_entry(&desktop_file) {
            if let Some(icon_found) = lookup_icon_path(&icon_name, ICON_SIZE, ICON_THEME) {
                return icon_found;
            };
        }
    };

    PathBuf::from(DEFAULT_ICON)
}

// follows the same strategy as waybar
// https://github.com/Alexays/Waybar/tree/master/src/modules/wlr/taskbar.cpp
fn guess_desktop_file(app_class: &str) -> Option<PathBuf> {
    let mut desktop_files = freedesktop_desktop_entry::Iter::new(default_paths());

    desktop_files.find(|path| {
        if let Some(name) = path.file_stem() {
            if let Some(name) = name.to_str() {
                if name.to_lowercase() == app_class.to_lowercase() {
                    return true;
                };

                // org.domain.app_class
                if let Some(start) = app_class.rfind('.').map(|n| n + 1) {
                    if name.to_lowercase() == app_class[start..].to_lowercase() {
                        return true;
                    }
                }

                if let Some(start) = name.rfind('.').map(|n| n + 1) {
                    if name[start..].to_lowercase() == app_class.to_lowercase() {
                        return true;
                    }
                }

                // app_class-bla-bla-bla
                if let Some(end) = app_class.find('-') {
                    if name.to_lowercase() == app_class[..end].to_lowercase() {
                        return true;
                    }
                }
            }
        };

        false
    })
}

fn get_icon_from_desktop_entry(desktop_file: &PathBuf) -> Option<String> {
    if let Ok(bytes) = fs::read_to_string(desktop_file) {
        if let Ok(entry) = DesktopEntry::decode(desktop_file, &bytes) {
            return Some(entry.icon()?.to_string());
        }
    }

    None
}

fn lookup_icon_path(icon_name: &str, size: u16, theme: &str) -> Option<PathBuf> {
    freedesktop_icons::lookup(icon_name)
        .with_size(size)
        .with_theme(theme)
        .with_cache()
        .find()
}
