#[derive(Debug,Clone)]
pub struct Shortcut<'a> {    
    pub order: usize,
    pub app_id: u32,
    pub app_name: &'a str,
    pub exe: &'a str,
    pub start_dir: &'a str,
    pub icon: &'a str,
    pub shortcut_path: &'a str,
    pub launch_options: &'a str,
    pub is_hidden: bool,
    pub allow_desktop_config: bool,
    pub allow_overlay: bool,
    pub open_vr: u32,
    pub dev_kit: u32,
    pub dev_kit_game_id: &'a str,
    pub last_play_time: u32,
    pub tags: Vec<&'a str>,
}