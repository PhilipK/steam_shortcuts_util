use nom::bytes::complete::{tag, take, take_till};
use nom::multi::many0;
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

fn parse_shortcuts_inner<'a>(
    shortcuts_bytes: &'a [u8],
) -> nom::IResult<&[u8], Vec<Shortcut<'a>>> {
    let (i, _) = shotcut_content(shortcuts_bytes)?;
    let (i, list) = many0(get_shortcut)(i)?;
    let bs = ascii::AsciiChar::BackSpace.as_byte();
    let (i, _) = tag([bs])(i)?;
    IResult::Ok((i, list))
}

fn get_shortcut<'a>(i: &'a [u8]) -> nom::IResult<&[u8], Shortcut<'a>> {
    let (i, order) = get_order(i)?;
    let (i, app_id) = get_app_id(i)?;
    let (i, app_name) = get_app_name(i)?;
    let (i, exe) = get_exe(i)?;
    let (i, start_dir) = get_start_dir(i)?;
    let (i, icon) = get_icon(i)?;
    let (i, shortcut_path) = get_shortcut_path(i)?;
    let (i, launch_options) = get_launch_options(i)?;
    let (i, is_hidden) = get_is_hidden(i)?;
    let (i, allow_desktop_config) = get_allow_desktop_config(i)?;
    let (i, allow_overlay) = get_allow_overlay(i)?;
    let (i, open_vr) = get_open_vr(i)?;
    let (i, dev_kit) = get_dev_kit(i)?;
    let (i, dev_kit_game_id) = get_devkit_game_id(i)?;
    let (i, last_play_time) = get_last_time_played(i)?;
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
        },
    ))
}

fn get_order(i: &[u8]) -> nom::IResult<&[u8], usize> {
    let null = ascii::AsciiChar::Null.as_byte();
    let (i, _) = tag([null])(i)?;
    let (i, order_bytes) = take_till(|c| c == null)(i)?;
    let (i, _) = tag([null])(i)?;
    let order_string = std::str::from_utf8(&order_bytes).unwrap();
    let order = order_string.parse::<usize>().unwrap();
    IResult::Ok((i, order))
}

fn get_app_id(i: &[u8]) -> nom::IResult<&[u8], u32> {
    stx_line_parser("appid", i)
}

fn get_is_hidden(i: &[u8]) -> nom::IResult<&[u8], bool> {
    stx_line_parser("IsHidden", i).map(|(u, val)| (u, val != 0))
}

fn get_allow_desktop_config(i: &[u8]) -> nom::IResult<&[u8], bool> {
    stx_line_parser("AllowDesktopConfig", i).map(|(u, val)| (u, val != 0))
}

fn get_allow_overlay(i: &[u8]) -> nom::IResult<&[u8], bool> {
    stx_line_parser("AllowOverlay", i).map(|(u, val)| (u, val != 0))
}

fn get_open_vr(i: &[u8]) -> nom::IResult<&[u8], u32> {
    stx_line_parser("openvr", i)
}

fn get_dev_kit(i: &[u8]) -> nom::IResult<&[u8], u32> {
    stx_line_parser("Devkit", i)
}

fn get_last_time_played(i: &[u8]) -> nom::IResult<&[u8], u32> {
    stx_line_parser("LastPlayTime", i)
}

fn get_devkit_game_id(i: &[u8]) -> nom::IResult<&[u8], &str> {
    soh_line_parser("DevkitGameID")(i)
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
    let null = ascii::AsciiChar::Null.as_byte();
    let soh = ascii::AsciiChar::SOH.as_byte();

    let (i, _) = tag([soh])(i)?;
    let (i, _) = take_till(|c| c == null)(i)?;
    let (i, _) = tag([null])(i)?;
    let (i, tag_name_bytes) = take_till(|c| c == null)(i)?;
    let (i, _) = tag([null])(i)?;
    let tag_name = std::str::from_utf8(&tag_name_bytes).unwrap();
    IResult::Ok((i, tag_name))
}

fn stx_line_parser<'b>(key: &str, i: &'b [u8]) -> nom::IResult<&'b [u8], u32> {
    use nom::branch::alt;
    alt((stx_single_line_parser(key), stx_4_line_parser(key)))(i)
}

fn stx_4_line_parser(key: &str) -> impl Fn(&[u8]) -> nom::IResult<&[u8], u32> {
    let owned_key = key.to_owned();
    move |i: &[u8]| {
        use nom::sequence::tuple;
        let stx = ascii::AsciiChar::SOX.as_byte();
        let null = ascii::AsciiChar::Null.as_byte();
        let (i, (_, _, _, app_id_bytes)) = tuple((
            tag([stx]),
            tag(owned_key.as_str()),
            tag([null]),
            take(4usize),
        ))(i)?;
        let app_id_bytes_slized: [u8; 4] = [
            app_id_bytes[0],
            app_id_bytes[1],
            app_id_bytes[2],
            app_id_bytes[3],
        ];

        let app_id = u32::from_le_bytes(app_id_bytes_slized);
        IResult::Ok((i, app_id))
    }
}

fn stx_single_line_parser(key: &str) -> impl Fn(&[u8]) -> nom::IResult<&[u8], u32> {
    let owned_key = key.to_owned();
    move |i: &[u8]| {
        use nom::sequence::tuple;
        let stx = ascii::AsciiChar::SOX.as_byte();
        let null = ascii::AsciiChar::Null.as_byte();
        let soh = ascii::AsciiChar::SOH.as_byte();

        let mut short_form = tuple((
            tag([stx]),
            tag(owned_key.as_str()),
            tag([null]),
            tag([soh]),
            take(3usize),
        ));
        let (i, (_, _, _, _, app_id_bytes)) = short_form(i)?;
        let app_id_bytes_slized: [u8; 4] =
            [0x00, app_id_bytes[0], app_id_bytes[1], app_id_bytes[2]];
        let app_id = u32::from_le_bytes(app_id_bytes_slized);
        IResult::Ok((i, app_id))
    }
}

fn get_app_name(i: &[u8]) -> nom::IResult<&[u8], &str> {
    soh_line_parser("AppName")(i)
}

fn get_exe(i: &[u8]) -> nom::IResult<&[u8], &str> {
    soh_line_parser("Exe")(i)
}

fn get_start_dir(i: &[u8]) -> nom::IResult<&[u8], &str> {
    soh_line_parser("StartDir")(i)
}

fn get_icon(i: &[u8]) -> nom::IResult<&[u8], &str> {
    soh_line_parser("icon")(i)
}

fn get_shortcut_path(i: &[u8]) -> nom::IResult<&[u8], &str> {
    soh_line_parser("ShortcutPath")(i)
}

fn get_launch_options(i: &[u8]) -> nom::IResult<&[u8], &str> {
    soh_line_parser("LaunchOptions")(i)
}

fn soh_line_parser(key: &str) -> impl Fn(&[u8]) -> nom::IResult<&[u8], &str> {
    let owned_key = key.to_owned();
    move |i: &[u8]| {
        let soh = ascii::AsciiChar::SOH.as_byte();
        let null = ascii::AsciiChar::Null.as_byte();
        let (i, _) = tag([soh])(i)?;
        let (i, _) = tag(owned_key.as_str())(i)?;
        let (i, _) = tag([null])(i)?;
        let (i, exe_name) = take_till(|c| c == null)(i)?;
        let (i, _) = tag([null])(i)?;
        IResult::Ok((i, std::str::from_utf8(exe_name).unwrap()))
    }
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
        assert_eq!(true, res.is_ok());
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
        let (r, id) = get_app_id(&i).unwrap();
        assert_eq!(2, r.len());
        assert_eq!(2365067149, id);
    }

    #[test]
    fn get_app_name_test() {
        const DATA: [u8; 17] = [
            // Offset 0x00000000 to 0x00000016
            0x01, 0x41, 0x70, 0x70, 0x4E, 0x61, 0x6D, 0x65, 0x00, 0x43, 0x65, 0x6C, 0x65, 0x73,
            0x74, 0x65, 0x00,
        ];
        let i = DATA;
        let res = get_app_name(&i);
        assert_eq!("Celeste", res.unwrap().1);
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
    fn get_content_from_file_test() {
        let content = std::fs::read("src/testdata/shortcuts.vdf").unwrap();
        let slice = content.as_slice();
        let (i, _) = shotcut_content(slice).unwrap();
        let (i, order) = get_order(i).unwrap();
        assert_eq!(0, order);
        let (i, app_id) = get_app_id(i).unwrap();
        assert_eq!(2365067149, app_id);
        let (i, name) = get_app_name(i).unwrap();
        assert_eq!("Celeste", name);
        let (i, exe) = get_exe(i).unwrap();
        assert_eq!("\"C:\\MySmallPrograms\\epic_launcher.exe\"", exe);
        let (i, start_dir) = get_start_dir(i).unwrap();
        assert_eq!("\"C:\\MySmallPrograms\\\"", start_dir);
        let (i, icon) = get_icon(i).unwrap();
        assert_eq!("\"F:\\EpicExtra\\Celeste\\Celeste.exe\"", icon);
        let (i, shortcut_path) = get_shortcut_path(i).unwrap();
        assert_eq!("", shortcut_path);
        let (i, lanch_options) = get_launch_options(i).unwrap();
        assert_eq!("Celeste.exe com.epicgames.launcher://apps/b671fbc7be424e888c9346a9a6d3d9db%3A38c07a09dc174b69b756aa51890c3dd4%3ASalt?action=launch&silent=true", lanch_options);
        let (i, is_hidden) = get_is_hidden(i).unwrap();
        assert_eq!(false, is_hidden);
        let (i, allow_desktop_config) = get_allow_desktop_config(i).unwrap();
        assert_eq!(false, allow_desktop_config);
        let (i, allow_overlay) = get_allow_overlay(i).unwrap();
        assert_eq!(false, allow_overlay);
        let (i, open_vr) = get_open_vr(i).unwrap();
        assert_eq!(0, open_vr);
        let (i, dev_kit) = get_dev_kit(i).unwrap();
        assert_eq!(0, dev_kit);
        let (i, dev_kit_game_id) = get_devkit_game_id(i).unwrap();
        assert_eq!("", dev_kit_game_id);
        let (i, last_time_played) = get_last_time_played(i).unwrap();
        assert_eq!(1628913700, last_time_played);
        let (i, tags) = get_tags(i).unwrap();
        assert_eq!(3, tags.len());
        let bs = ascii::AsciiChar::BackSpace.as_byte();
        assert_eq!(i[0], bs);
        assert_ne!(i[1], bs);
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
        let res = get_exe(&i);
        assert_eq!("\"C:\\MySmallPrograms\\epic_launcher.exe\"", res.unwrap().1);
    }

    #[test]
    fn get_is_hidden_test() {
        const DATA: [u8; 14] = [
            // Offset 0x00000332 to 0x00000345
            0x02, 0x49, 0x73, 0x48, 0x69, 0x64, 0x64, 0x65, 0x6E, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let i = DATA;
        let res = get_is_hidden(&i);
        assert_eq!(false, res.unwrap().1);
    }

    #[test]
    fn get_allow_desktop_config_test() {
        const DATA: [u8; 24] = [
            // Offset 0x00000346 to 0x00000369
            0x02, 0x41, 0x6C, 0x6C, 0x6F, 0x77, 0x44, 0x65, 0x73, 0x6B, 0x74, 0x6F, 0x70, 0x43,
            0x6F, 0x6E, 0x66, 0x69, 0x67, 0x00, 0x01, 0x00, 0x00, 0x00,
        ];

        let i = DATA;
        let res = get_allow_desktop_config(&i);
        assert_eq!(false, res.unwrap().1);
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
