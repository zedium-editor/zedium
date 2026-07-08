//! The Zedium facade: the single entry point for all Zedium fork features.
//!
//! Fork features live in their own `zedium_*` crates and are wired into the
//! app exclusively through [`init`], keeping the upstream patch surface to a
//! handful of hook lines. Features are compiled in and toggled at runtime via
//! the `zedium` key in `settings.json` (see `ZediumSettingsContent`).
//!
//! The full delta against upstream Zed is visible at
//! <https://github.com/zedium-editor/zedium/compare/main...zedium>.

use gpui::App;
use settings::{RegisterSetting, Settings, SettingsContent};

/// Top-level settings for Zedium fork features, read from the `zedium` key
/// in `settings.json`.
#[derive(RegisterSetting)]
pub struct ZediumSettings {
    pub enabled: bool,
}

impl Settings for ZediumSettings {
    fn from_settings(content: &SettingsContent) -> Self {
        let zedium = content.zedium.as_ref();
        Self {
            enabled: zedium.and_then(|zedium| zedium.enabled).unwrap_or(true),
        }
    }
}

impl ZediumSettings {
    /// Whether Zedium fork features are enabled at all. Feature crates must
    /// check this in addition to their own toggle.
    pub fn enabled(cx: &App) -> bool {
        Self::try_get(cx)
            .map(|settings| settings.enabled)
            .unwrap_or(true)
    }
}

/// Initializes all Zedium fork features. Called once from `main`, after
/// `settings::init`.
pub fn init(_cx: &mut App) {
    // Feature crates hook in here as they land, gated on ZediumSettings.
}
