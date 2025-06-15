use serde::{Deserialize, Serialize};

//FIX: 1.fuck this name...rename it
//2.use a parser
#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy)]
pub struct SkipCheck {
    #[serde(default, skip_serializing_if = "super::is_false")]
    pub target_exists: bool,
    #[serde(default, skip_serializing_if = "super::is_false")]
    pub source_is_git: bool,
    #[serde(default, skip_serializing_if = "super::is_false")]
    pub unresolved_env: bool,
    #[serde(default, skip_serializing_if = "super::is_false")]
    pub duplicated_config: bool,
}

impl SkipCheck {
    pub fn new() -> Self {
        SkipCheck {
            ..Default::default()
        }
    }
}
