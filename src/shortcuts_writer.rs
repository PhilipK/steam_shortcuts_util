use crate::shortcut::Shortcut;

use ascii::AsciiChar::*;

/// Serializes shortcuts to bytes, in a format that Steam will accept.
///
/// ### Examples
/// ```
/// use steam_shortcuts_util::parse_shortcuts;
/// use steam_shortcuts_util::shortcuts_to_bytes;
///
/// fn example() -> Result<(), Box<dyn std::error::Error>> {
///     // This path should be to your steams shortcuts file
///     // Usually located at $SteamDirectory/userdata/$SteamUserId/config/shortcuts.vdf
///     let content = std::fs::read("src/testdata/shortcuts.vdf")?;
///     let shortcuts = parse_shortcuts(content.as_slice())?;
///     let shortcut_bytes_vec = shortcuts_to_bytes(&shortcuts);
///     assert_eq!(shortcut_bytes_vec, content);
///     Ok(())
/// }
/// ```
pub fn shortcuts_to_bytes(shortcut: &Vec<Shortcut>) -> Vec<u8> {
    let null = Null.as_byte();

    let bs = BackSpace.as_byte();
    let mut result = vec![];

    result.push(null);
    result.extend_from_slice("shortcuts".as_bytes());
    result.push(null);

    let mut shortcut_bytes: Vec<u8> = shortcut
        .iter()
        .enumerate()
        .flat_map(|(index, shortcut)| shortcut_to_bytes(index, shortcut))
        .collect();

    result.append(&mut shortcut_bytes);

    result.push(bs);
    result.push(bs);

    result
}

fn shortcut_to_bytes(order: usize, shortcut: &Shortcut) -> Vec<u8> {
    let null = Null.as_byte();
    let bs = BackSpace.as_byte();

    let mut res = vec![];

    res.push(null);

    let order_string = format!("{}", order);
    let order = order_string.as_bytes();
    res.extend_from_slice(order);
    res.push(null);

    res.append(&mut stx_to_bytes("appid", shortcut.app_id));
    res.append(&mut soh_to_bytes("AppName", shortcut.app_name));
    res.append(&mut soh_to_bytes("Exe", shortcut.exe));
    res.append(&mut soh_to_bytes("StartDir", shortcut.start_dir));
    res.append(&mut soh_to_bytes("icon", shortcut.icon));
    res.append(&mut soh_to_bytes("ShortcutPath", shortcut.shortcut_path));
    res.append(&mut soh_to_bytes("LaunchOptions", shortcut.launch_options));
    res.append(&mut stx_to_bytes("IsHidden", shortcut.is_hidden as u32));
    res.append(&mut stx_single_to_bytes(
        "AllowDesktopConfig",
        shortcut.allow_desktop_config,
    ));
    res.append(&mut stx_single_to_bytes(
        "AllowOverlay",
        shortcut.allow_overlay,
    ));
    res.append(&mut stx_to_bytes("openvr", shortcut.open_vr));
    res.append(&mut stx_to_bytes("Devkit", shortcut.dev_kit));
    res.append(&mut soh_to_bytes("DevkitGameID", shortcut.dev_kit_game_id));
    res.append(&mut stx_to_bytes(
        "DevkitOverrideAppID",
        shortcut.dev_kit_overrite_app_id,
    ));

    res.append(&mut stx_to_bytes("LastPlayTime", shortcut.last_play_time));

    res.push(null);
    res.extend_from_slice("tags".as_bytes());
    res.push(null);

    res.append(&mut tags_to_bytes(&shortcut.tags));

    res.push(bs);
    res.push(bs);

    res
}

fn tags_to_bytes(input: &Vec<&str>) -> Vec<u8> {
    input
        .iter()
        .enumerate()
        .flat_map(|(index, tag)| tag_to_bytes(index, tag))
        .collect()
}

fn tag_to_bytes(index: usize, tag: &str) -> Vec<u8> {
    let soh = SOH.as_byte();
    let null = Null.as_byte();

    let mut res = vec![];
    res.push(soh);
    let order_string = format!("{}", index);
    let order = order_string.as_bytes();
    res.extend_from_slice(order);
    res.push(null);
    res.extend_from_slice(tag.as_bytes());
    res.push(null);
    res
}

fn soh_to_bytes(name: &str, input: &str) -> Vec<u8> {
    let mut res = vec![];
    let soh = SOH.as_byte();
    let null = Null.as_byte();
    res.push(soh);

    res.extend_from_slice(name.as_bytes());

    res.push(null);
    res.extend_from_slice(&input.as_bytes());
    res.push(null);
    res
}

fn stx_single_to_bytes(name: &str, input: bool) -> Vec<u8> {
    let mut res = vec![];
    let soh = SOH.as_byte();
    let stx = SOX.as_byte();
    let null = Null.as_byte();
    res.push(stx);

    res.extend_from_slice(name.as_bytes());
    res.push(null);
    res.push(soh);
    res.push(null);
    res.push(null);
    res.push(input as u8);
    res
}

fn stx_to_bytes(name: &str, input: u32) -> Vec<u8> {
    let mut res = vec![];
    let stx = SOX.as_byte();
    let null = Null.as_byte();
    res.push(stx);

    res.extend_from_slice(name.as_bytes());

    res.push(null);
    res.extend_from_slice(&input.to_le_bytes());
    res
}
#[cfg(test)]
mod tests {

    use crate::{shortcuts_parser, shortcuts_to_bytes};

    #[test]
    fn parse_back_and_forth() {
        let content = std::fs::read("src/testdata/shortcuts.vdf").unwrap();
        let shortcuts = shortcuts_parser::parse_shortcuts(content.as_slice()).unwrap();
        let shortcut_bytes_vec = shortcuts_to_bytes(&shortcuts);
        let shortcuts_again =
            shortcuts_parser::parse_shortcuts(shortcut_bytes_vec.as_slice()).unwrap();
        assert_eq!(shortcuts, shortcuts_again);
    }

    #[test]
    fn parse_back_and_forth_linux() {
        let content = std::fs::read("src/testdata/linux_shortcut.vdf").unwrap();
        let shortcuts = shortcuts_parser::parse_shortcuts(content.as_slice()).unwrap();
        let shortcut_bytes_vec = shortcuts_to_bytes(&shortcuts);
        let shortcuts_again =
            shortcuts_parser::parse_shortcuts(shortcut_bytes_vec.as_slice()).unwrap();
        assert_eq!(shortcuts, shortcuts_again);
    }

    #[test]
    fn parse_back_and_forth_firefox() {
        let content = std::fs::read("src/testdata/shortcutsfirefox.vdf").unwrap();
        let shortcuts = shortcuts_parser::parse_shortcuts(content.as_slice()).unwrap();
        let shortcut_bytes_vec = shortcuts_to_bytes(&shortcuts);
        let shortcuts_again =
            shortcuts_parser::parse_shortcuts(shortcut_bytes_vec.as_slice()).unwrap();
        assert_eq!(shortcuts, shortcuts_again);
    }
}
