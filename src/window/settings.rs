use crate::{cmd_line::CmdLineSettings, settings::*};

#[derive(Clone, SettingGroup)]
pub struct WindowSettings {
    pub refresh_rate: u64,
    pub no_idle: bool,
    pub transparency: f32,
    pub fullscreen: bool,
    pub iso_layout: bool,
    pub remember_window_size: bool,
    pub remember_window_position: bool,
    pub hide_mouse_when_typing: bool,
    pub touch_deadzone: f32,
    pub touch_drag_timeout: f32,
    pub top_padding: u32,
    pub left_padding: u32,
    pub right_padding: u32,
    pub bottom_padding: u32,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            transparency: 1.0,
            fullscreen: false,
            iso_layout: false,
            refresh_rate: 60,
            no_idle: SETTINGS.get::<CmdLineSettings>().no_idle,
            remember_window_size: true,
            remember_window_position: true,
            hide_mouse_when_typing: false,
            touch_deadzone: 6.0,
            touch_drag_timeout: 0.17,
            top_padding: 0,
            left_padding: 0,
            right_padding: 0,
            bottom_padding: 0,
        }
    }
}

#[derive(Clone, Default, SettingGroup)]
#[setting_prefix = "input"]
pub struct KeyboardSettings {
    pub use_logo: bool,
}
