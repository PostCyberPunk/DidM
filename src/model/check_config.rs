use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//FIX:
//2.use a parser maybe?
#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, JsonSchema)]
#[serde(rename = "skip_check")]
pub struct CheckConfig {
    #[serde(default, skip_serializing_if = "super::is_false")]
    pub is_git_workspace: bool,
    #[serde(default, skip_serializing_if = "super::is_false")]
    //WARN:that's a flip!!
    pub unresolved_env: bool,
    #[serde(default, skip_serializing_if = "super::is_false")]
    pub is_working_dir_symlink: bool,
    // #[serde(default, skip_serializing_if = "super::is_false")]
    // pub outside_home: bool,
    // #[serde(default, skip_serializing_if = "super::is_false")]
    // pub target_exists: bool,
    // #[serde(default, skip_serializing_if = "super::is_false")]
    // pub duplicated_config: bool,
}

impl CheckConfig {
    pub fn new() -> Self {
        CheckConfig {
            ..Default::default()
        }
    }
}
