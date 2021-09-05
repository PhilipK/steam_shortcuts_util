use std::borrow::Borrow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]

/// Struct with data for a steam shortcut.vdf file.
pub struct Shortcut<'a> {
    /// Order/Index  of the shortcut
    pub order: usize,
    /// The id for this shortcut.
    ///
    /// This id can be generated for a shortcut (in a way that steam will accept) with
    /// [calculate_app_id](crate::calculate_app_id)
    pub app_id: u32,
    /// The app name for this shortcut.
    pub app_name: &'a str,
    /// the target location
    pub exe: &'a str,
    /// The directory to launch the shortcut in (also known as working directory).
    pub start_dir: &'a str,
    /// Path to the icon of the shortcut
    pub icon: &'a str,
    /// The path to the shortcut.
    pub shortcut_path: &'a str,
    /// Options to pass to the exe in the target location
    pub launch_options: &'a str,
    /// Is this shortcut hidden?
    pub is_hidden: bool,
    /// Is dekstop configuration allowed
    pub allow_desktop_config: bool,
    /// Are steam overlays allowed
    pub allow_overlay: bool,
    /// Open vr id
    pub open_vr: u32,
    /// Devkit id
    pub dev_kit: u32,
    /// Devkit game id
    pub dev_kit_game_id: &'a str,
    /// Devkit overrite_app_id
    pub dev_kit_overrite_app_id: u32,
    /// The last time played in u32 seconds
    pub last_play_time: u32,
    /// A list of tags for this shortcut
    ///
    /// The tags: "Installed", "Ready TO Play" are recommended
    pub tags: Vec<&'a str>,
}

/// Struct with data for a steam shortcut.vdf file.
/// This struct owns the data it is referecing.
pub struct ShortcutOwned {
    /// Order/Index  of the shortcut
    pub order: usize,
    /// The id for this shortcut.
    ///
    /// This id can be generated for a shortcut (in a way that steam will accept) with
    /// [calculate_app_id](crate::calculate_app_id)
    pub app_id: u32,
    /// The app name for this shortcut.
    pub app_name: String,
    /// the target location
    pub exe: String,
    /// The directory to launch the shortcut in (also known as working directory).
    pub start_dir: String,
    /// Path to the icon of the shortcut
    pub icon: String,
    /// The path to the shortcut.
    pub shortcut_path: String,
    /// Options to pass to the exe in the target location
    pub launch_options: String,
    /// Is this shortcut hidden?
    pub is_hidden: bool,
    /// Is dekstop configuration allowed
    pub allow_desktop_config: bool,
    /// Are steam overlays allowed
    pub allow_overlay: bool,
    /// Open vr id
    pub open_vr: u32,
    /// Devkit id
    pub dev_kit: u32,
    /// Devkit game id
    pub dev_kit_game_id: String,
    /// Devkit overrite_app_id
    pub dev_kit_overrite_app_id: u32,
    /// The last time played in u32 seconds
    pub last_play_time: u32,
    /// A list of tags for this shortcut
    ///
    /// The tags: "Installed", "Ready TO Play" are recommended
    pub tags: Vec<String>,
}

impl ShortcutOwned {
    pub fn borrow<'a>(&'a self) -> Shortcut<'a> {
        Shortcut {
            order: self.order,
            app_id: self.app_id,
            app_name: &self.app_name,
            exe: &self.exe,
            start_dir: &self.start_dir,
            icon: &self.icon,
            shortcut_path: &self.shortcut_path,
            launch_options: &self.launch_options,
            is_hidden: self.is_hidden,
            allow_desktop_config: self.allow_desktop_config,
            allow_overlay: self.allow_overlay,
            open_vr: self.open_vr,
            dev_kit: self.dev_kit,
            dev_kit_game_id: &self.dev_kit_game_id,
            dev_kit_overrite_app_id: self.dev_kit_overrite_app_id,
            last_play_time: self.last_play_time,
            tags: self.tags.iter().map(|x| x.as_str()).collect(),
        }
    }
}

impl<'a> Shortcut<'a> {
    /// Create a new shortcut with sensible defaults.
    ///
    /// # Arguments
    ///
    /// * `order` - The order/index of the shortcut
    /// * `app_name` - The name of the shortcut
    /// * `exe` - The target location
    /// * `start_dir` - The directory to launch the shortcut in (also known as working directory)
    /// * `icon` - Path to the icon of the shortcut
    /// * `shortcut_path` - The path to the shortcut.
    /// * `launch_options` - Options to pass to the exe in the target location
    pub fn new(
        order: usize,
        app_name: &'a str,
        exe: &'a str,
        start_dir: &'a str,
        icon: &'a str,
        shortcut_path: &'a str,
        launch_options: &'a str,
    ) -> Self {
        let app_id = crate::app_id_generator::calculate_app_id(exe, app_name);
        let is_hidden = false;
        let allow_desktop_config = true;
        let allow_overlay = true;
        let open_vr = 0;
        let dev_kit = 0;
        let dev_kit_game_id = "";
        let last_play_time = 0;
        let dev_kit_overrite_app_id = 0;
        let tags = vec!["Installed", "Ready To Play"];
        Self {
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
            dev_kit_overrite_app_id,
            tags,
        }
    }

    pub fn to_owned(&self) -> ShortcutOwned {
        let owned_tags = self.tags.iter().map(|s| s.to_string()).collect();
        ShortcutOwned {
            order: self.order,
            app_id: self.app_id,
            app_name: self.app_name.to_owned(),
            exe: self.exe.to_owned(),
            start_dir: self.start_dir.to_owned(),
            icon: self.icon.to_owned(),
            shortcut_path: self.shortcut_path.to_owned(),
            launch_options: self.launch_options.to_owned(),
            is_hidden: self.is_hidden,
            allow_desktop_config: self.allow_desktop_config,
            allow_overlay: self.allow_overlay,
            open_vr: self.open_vr,
            dev_kit: self.dev_kit,
            dev_kit_game_id: self.dev_kit_game_id.to_owned(),
            dev_kit_overrite_app_id: self.dev_kit_overrite_app_id,
            last_play_time: self.last_play_time,
            tags: owned_tags,
        }
    }
}
