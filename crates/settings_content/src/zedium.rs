use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

/// Settings for Zedium fork features, under the top-level `zedium` key.
///
/// Each fork feature crate adds its own field here so features can be
/// toggled at runtime from `settings.json` without rebuilding.
#[with_fallible_options]
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug, PartialEq)]
pub struct ZediumSettingsContent {
    /// Master switch for all Zedium fork features.
    ///
    /// Default: true
    pub enabled: Option<bool>,
}
