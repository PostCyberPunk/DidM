use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy)]
pub struct Check {
    #[serde(default, skip_serializing_if = "super::is_false")]
    pub target_exists: bool,
    #[serde(default, skip_serializing_if = "super::is_false")]
    pub source_is_git: bool,
    #[serde(default, skip_serializing_if = "super::is_false")]
    pub unresolved_env: bool,
    #[serde(default, skip_serializing_if = "super::is_false")]
    pub duplicated_config: bool,
}

impl Check {
    pub fn new() -> Self {
        Check {
            ..Default::default()
        }
    }
}
