use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ValidationProfile {
    #[default]
    Strict,
    Structural,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecodeOptions {
    pub validation_profile: ValidationProfile,
    pub allow_unknown_event_types: bool,
}

impl Default for DecodeOptions {
    fn default() -> Self {
        Self {
            validation_profile: ValidationProfile::Strict,
            allow_unknown_event_types: false,
        }
    }
}
