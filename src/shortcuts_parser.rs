use std::collections::HashMap;

use nom::bytes::complete::{tag, take, take_till};
use nom::multi::{many0, many1};
use nom::IResult;

use crate::shortcut::Shortcut;

/// Parse bytes to shortcuts, if the bytes are in a format of the shortcuts.vdf file.
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
///     assert_eq!(shortcuts[0].app_name, "Celeste");
///     assert_eq!(3, shortcuts[0].tags.len());
///     Ok(())
/// }
/// ```
pub fn parse_shortcuts<'a>(shortcuts_bytes: &'a [u8]) -> Result<Vec<Shortcut<'a>>, String> {
    match parse_shortcuts_inner(shortcuts_bytes) {
        Ok((_, shortcuts)) => Result::Ok(shortcuts),
        Err(err) => Result::Err(format!("{}", err)),
    }
}

fn get_shortcut<'a>(i: &'a [u8]) -> nom::IResult<&[u8], Shortcut<'a>> {
    let (i, order) = get_order(i)?;

    let (i, lines) = parse_all_lines(i)?;

    let numeric_value = |name: &str| lines.get(name).map(|l| l.num_value()).unwrap_or_default();
    let text_value = |name: &str| lines.get(name).map(|l| l.text_value()).unwrap_or_default();

    let app_id = numeric_value("app_id");
    let app_name = text_value("AppName");
    let exe = text_value("Exe");
    let start_dir = text_value("StartDir");
    let icon = text_value("icon");
    let shortcut_path = text_value("ShortcutPath");
    let launch_options = text_value("LaunchOptions");
    let is_hidden = numeric_value("IsHidden") != 0;
    let allow_desktop_config = numeric_value("AllowDesktopConfig") != 0;
    let allow_overlay = numeric_value("AllowOverlay") != 0;
    let open_vr = numeric_value("openvr");
    let dev_kit = numeric_value("Devkit");

    let dev_kit_game_id = text_value("DevkitGameID");
    let dev_kit_overrite_app_id = numeric_value("DevkitOverrideAppID");
    let last_play_time = numeric_value("LastPlayTime");

    let (i, tags) = get_tags(i)?;
    let bs = ascii::AsciiChar::BackSpace.as_byte();
    let (i, _) = tag([bs])(i)?;
    IResult::Ok((
        i,
        Shortcut {
            order,
            app_id,
            app_name,
            exe,
            start_dir,
            icon,
            shortcut_path,
            launch_options,
            is_hidden,
            allow_desktop_config,
            allow_overlay,
            open_vr,
            dev_kit,
            dev_kit_game_id,
            last_play_time,
            tags,
            dev_kit_overrite_app_id,
        },
    ))
}

fn parse_shortcuts_inner<'a>(shortcuts_bytes: &'a [u8]) -> nom::IResult<&[u8], Vec<Shortcut<'a>>> {
    let (i, _) = shotcut_content(shortcuts_bytes)?;
    let (i, list) = many0(get_shortcut)(i)?;
    let bs = ascii::AsciiChar::BackSpace.as_byte();
    let (i, _) = tag([bs])(i)?;
    IResult::Ok((i, list))
}

pub enum LineType<'a> {
    Text { name: &'a str, value: &'a str },
    Numeric { name: &'a str, value: u32 },
}

impl<'a> LineType<'a> {
    fn name(&self) -> &'a str {
        match *self {
            LineType::Text { name, value: _ } => name,
            LineType::Numeric { name, value: _ } => name,
        }
    }

    fn text_value(&self) -> &'a str {
        match *self {
            LineType::Text { name: _, value } => value,
            LineType::Numeric { name: _, value: _ } => "",
        }
    }

    fn num_value(&self) -> u32 {
        match *self {
            LineType::Text { name: _, value: _ } => 0,
            LineType::Numeric { name: _, value } => value,
        }
    }
}

fn parse_all_lines<'a>(i: &'a [u8]) -> nom::IResult<&'a [u8], HashMap<&'a str, LineType<'a>>> {
    let (i, list) = many1(parse_a_line)(i)?;
    let mut res = HashMap::new();
    let list_iter = list.into_iter();
    list_iter.for_each(|l| {
        res.insert(l.name(), l);
    });
    IResult::Ok((i, res))
}

fn parse_a_line<'a>(i: &'a [u8]) -> nom::IResult<&'a [u8], LineType<'a>> {
    if let Ok((i, (name, value))) = parse_text_line(i) {
        return IResult::Ok((i, LineType::Text { name, value }));
    }
    let (i, (name, value)) = parse_numeric_line(i)?;
    return IResult::Ok((i, LineType::Numeric { name, value }));
}

fn parse_numeric_line<'b>(i: &'b [u8]) -> nom::IResult<&'b [u8], (&'b str, u32)> {
    let stx = ascii::AsciiChar::SOX.as_byte();

    let (i, _) = tag([stx])(i)?;
    let (i, key) = get_null_terminated_str(i)?;
    let (i, value) = get_a_u32(i)?;
    IResult::Ok((i, (key, value)))
}

fn parse_text_line<'a>(i: &'a [u8]) -> nom::IResult<&'a [u8], (&'a str, &'a str)> {
    let soh = ascii::AsciiChar::SOH.as_byte();
    let (i, _) = tag([soh])(i)?;
    let (i, key) = get_null_terminated_str(i)?;
    let (i, value) = get_null_terminated_str(i)?;
    IResult::Ok((i, (key, value)))
}

fn get_a_u32<'b>(i: &'b [u8]) -> nom::IResult<&'b [u8], u32> {
    use nom::branch::alt;
    alt((get_soh_u32, get_normal_u32))(i)
}

fn get_normal_u32<'b>(i: &'b [u8]) -> nom::IResult<&'b [u8], u32> {
    let (i, app_bytes) = take(4usize)(i)?;
    let app_id_bytes_slized: [u8; 4] = [app_bytes[0], app_bytes[1], app_bytes[2], app_bytes[3]];
    let app_id = u32::from_le_bytes(app_id_bytes_slized);
    IResult::Ok((i, app_id))
}

fn get_soh_u32<'b>(i: &'b [u8]) -> nom::IResult<&'b [u8], u32> {
    let soh = ascii::AsciiChar::SOH.as_byte();
    let (i, _) = tag([soh])(i)?;
    let (i, app_id_bytes) = take(3usize)(i)?;
    let app_id_bytes_slized: [u8; 4] = [0x00, app_id_bytes[0], app_id_bytes[1], app_id_bytes[2]];
    let app_id = u32::from_le_bytes(app_id_bytes_slized);
    IResult::Ok((i, app_id))
}

fn get_null_terminated_str<'a>(i: &'a [u8]) -> nom::IResult<&'a [u8], &'a str> {
    let null = ascii::AsciiChar::Null.as_byte();
    let (i, str_bytes) = take_till(|cond| cond == null)(i)?;

    //TODO Remove this unwrap
    let str_res = std::str::from_utf8(str_bytes).unwrap();
    let (i, _null) = tag([null])(i)?;
    IResult::Ok((i, str_res))
}

fn get_order(i: &[u8]) -> nom::IResult<&[u8], usize> {
    let null = ascii::AsciiChar::Null.as_byte();
    let (i, _) = tag([null])(i)?;
    let (i, order_string) = get_null_terminated_str(i)?;
    let order = order_string.parse::<usize>().unwrap();
    IResult::Ok((i, order))
}

fn get_tags(i: &[u8]) -> nom::IResult<&[u8], Vec<&str>> {
    use nom::sequence::tuple;

    let null = ascii::AsciiChar::Null.as_byte();
    let bs = ascii::AsciiChar::BackSpace.as_byte();
    let (i, (_, _, _, tags_bytes, _)) = tuple((
        tag([null]),
        tag("tags"),
        tag([null]),
        take_till(|c| c == bs),
        tag([bs]),
    ))(i)?;
    let (_, tags) = many0(take_tag)(tags_bytes)?;
    IResult::Ok((i, tags))
}

fn take_tag<'b>(i: &[u8]) -> nom::IResult<&[u8], &str> {
    let soh = ascii::AsciiChar::SOH.as_byte();

    let (i, _) = tag([soh])(i)?;
    let (i, _) = get_null_terminated_str(i)?;        
    let (i, tag_name) = get_null_terminated_str(i)?;        
    IResult::Ok((i, tag_name))
}

fn shotcut_content(i: &[u8]) -> nom::IResult<&[u8], ()> {
    use nom::character::complete::char;
    use nom::sequence::tuple;
    let null = ascii::AsciiChar::Null.as_char();
    let mut start_sequence = tuple((char(null), tag("shortcuts"), char(null)));
    let (x, _y) = start_sequence(i)?;
    IResult::Ok((x, ()))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_content() {
        let content = std::fs::read("src/testdata/shortcuts.vdf").unwrap();

        let res = shotcut_content(content.as_slice());
        let _unwrapped = res.unwrap();
    }

    #[test]
    fn parse_full_file() {
        let content = std::fs::read("src/testdata/shortcuts_broken.vdf").unwrap();

        let res = parse_shortcuts(content.as_slice());
        let _unwrapped = res.unwrap();
    }

    #[test]
    fn get_order_test() {
        const DATA: [u8; 3] = [
            // Offset 0x00000011 to 0x00000013
            0x00, 0x30, 0x00,
        ];
        let (r, order) = get_order(&DATA).unwrap();
        assert_eq!(0, order);
        assert_eq!(0, r.len());
    }

    #[test]
    fn get_app_id_test() {
        const DATA: [u8; 13] = [
            // Offset 0x00000000 to 0x00000010
            0x02, 0x61, 0x70, 0x70, 0x69, 0x64, 0x00, 0x8D, 0x0F, 0xF8, 0x8C, 0x01, 0x41,
        ];
        let i = DATA;
        let (r, id) = parse_a_line(&i).unwrap();
        assert_eq!(2, r.len());
        assert_eq!("appid", id.name());
        assert_eq!(2365067149, id.num_value());
    }

    #[test]
    fn get_app_name_test() {
        const DATA: [u8; 17] = [
            // Offset 0x00000000 to 0x00000016
            0x01, 0x41, 0x70, 0x70, 0x4E, 0x61, 0x6D, 0x65, 0x00, 0x43, 0x65, 0x6C, 0x65, 0x73,
            0x74, 0x65, 0x00,
        ];
        let i = DATA;
        let (_r, id) = parse_a_line(&i).unwrap();
        assert_eq!("AppName", id.name());
        assert_eq!("Celeste", id.text_value());
    }

    #[test]
    fn get_shortcuts_from_file() {
        let content = std::fs::read("src/testdata/shortcuts.vdf").unwrap();
        let slice = content.as_slice();
        let shortcuts = parse_shortcuts(slice).unwrap();
        assert_eq!(42, shortcuts.len());
    }

    #[test]
    fn get_shortcut_content_test() {
        let content = std::fs::read("src/testdata/shortcuts.vdf").unwrap();
        let slice = content.as_slice();
        let (i, _) = shotcut_content(slice).unwrap();
        let (i, s) = get_shortcut(i).unwrap();
        assert_eq!("Celeste", s.app_name);
        let (_i, s) = get_shortcut(i).unwrap();
        assert_eq!("Death Stranding", s.app_name);
    }

    #[test]
    fn get_shortcut_content_test_mixed() {
        let content = std::fs::read("src/testdata/shortcuts2.vdf").unwrap();
        let slice = content.as_slice();
        let (i, _) = shotcut_content(slice).unwrap();
        let (i, s) = get_shortcut(i).unwrap();
        assert_eq!("Celeste", s.app_name);
        let (_i, s) = get_shortcut(i).unwrap();
        assert_eq!("Death Stranding", s.app_name);
    }

    #[test]
    fn get_exe_name_test() {
        const DATA: [u8; 44] = [
            // Offset 0x00000042 to 0x00000085
            0x01, 0x45, 0x78, 0x65, 0x00, 0x22, 0x43, 0x3A, 0x5C, 0x4D, 0x79, 0x53, 0x6D, 0x61,
            0x6C, 0x6C, 0x50, 0x72, 0x6F, 0x67, 0x72, 0x61, 0x6D, 0x73, 0x5C, 0x65, 0x70, 0x69,
            0x63, 0x5F, 0x6C, 0x61, 0x75, 0x6E, 0x63, 0x68, 0x65, 0x72, 0x2E, 0x65, 0x78, 0x65,
            0x22, 0x00,
        ];

        let i = DATA;
        let (_r, id) = parse_a_line(&i).unwrap();
        assert_eq!("Exe", id.name());
        assert_eq!(
            "\"C:\\MySmallPrograms\\epic_launcher.exe\"",
            id.text_value()
        );
    }

    #[test]
    fn get_is_hidden_test() {
        const DATA: [u8; 14] = [
            // Offset 0x00000332 to 0x00000345
            0x02, 0x49, 0x73, 0x48, 0x69, 0x64, 0x64, 0x65, 0x6E, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let i = DATA;
        let (_r, id) = parse_a_line(&i).unwrap();
        assert_eq!(false, id.num_value() != 0);
    }

    #[test]
    fn get_allow_desktop_config_test() {
        const DATA: [u8; 24] = [
            // Offset 0x00000346 to 0x00000369
            0x02, 0x41, 0x6C, 0x6C, 0x6F, 0x77, 0x44, 0x65, 0x73, 0x6B, 0x74, 0x6F, 0x70, 0x43,
            0x6F, 0x6E, 0x66, 0x69, 0x67, 0x00, 0x01, 0x00, 0x00, 0x00,
        ];

        let i = DATA;
        let (_r, id) = parse_a_line(&i).unwrap();
        assert_eq!(false, id.num_value() != 0);
    }

    #[test]
    fn get_tags_test() {
        const DATA: [u8; 49] = [
            // Offset 0x00000445 to 0x00000493
            0x00, 0x74, 0x61, 0x67, 0x73, 0x00, 0x01, 0x30, 0x00, 0x66, 0x61, 0x76, 0x6F, 0x72,
            0x69, 0x74, 0x65, 0x00, 0x01, 0x31, 0x00, 0x49, 0x6E, 0x73, 0x74, 0x61, 0x6C, 0x6C,
            0x65, 0x64, 0x00, 0x01, 0x32, 0x00, 0x52, 0x65, 0x61, 0x64, 0x79, 0x20, 0x54, 0x4F,
            0x20, 0x50, 0x6C, 0x61, 0x79, 0x00, 0x08,
        ];

        let i = DATA;
        let res = get_tags(&i);
        let res_unwrapped = res.unwrap();
        assert_eq!(
            vec!["favorite", "Installed", "Ready TO Play"],
            res_unwrapped.1
        );
    }
}
