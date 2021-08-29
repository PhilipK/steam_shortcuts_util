use crc32fast::Hasher;

use crate::shortcut::Shortcut;

/// Calculate an app id for a shortcut.
/// The app id is a 32-bit hash of the shortcut exe path and its app_name.
/// It is used to identify custom images for the shortcut.
pub fn calculate_app_id_for_shortcut(shortcut: &Shortcut) -> u32 {
    calculate_app_id(shortcut.exe, shortcut.app_name)
}

/// Calculate an app id for a exe and app_name
/// The app id is a 32-bit hash of the shortcut exe path and its app_name.
/// It is used to identify custom images for the shortcut.
pub fn calculate_app_id(exe: &str, app_name: &str) -> u32 {
    let mut hasher = Hasher::new();
    let combined = format!("{}{}", exe, app_name);
    hasher.update(combined.as_bytes());
    let checksum = hasher.finalize();
    checksum | 0x80000000
}
