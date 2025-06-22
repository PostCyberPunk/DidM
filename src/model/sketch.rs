mod types;
pub use types::{Mode, Unit};

use crate::model::behaviour::Behaviour;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Default, JsonSchema)]
pub struct Sketch {
    pub source_path: String,
    pub target_path: String,
    #[serde(default, skip_serializing_if = "Mode::is_default")]
    pub mode: types::Mode,
    #[serde(default, skip_serializing_if = "Unit::is_default")]
    pub unit: types::Unit,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ignore: Vec<String>,
    //TODO: maybe i should use option intead of this pile of shit?
    //but then i have to call another function to determine its default value
    //so... emmm
    #[serde(
        default = "super::default_true",
        skip_serializing_if = "super::is_true"
    )]
    pub respect_gitignore: bool,
    #[serde(default, skip_serializing_if = "super::is_false")]
    pub ignore_hidden: bool,
    #[serde(default, skip_serializing_if = "super::is_false")]
    pub only_ignore: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub null_files: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub empty_files: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_build_commands: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub post_build_commands: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commands_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub override_behaviour: Option<Behaviour>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extra_entries: Vec<types::ExtraEntry>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub environment: HashMap<String, String>,
}
impl Sketch {
    pub fn new() -> Self {
        Sketch {
            source_path: String::from("."),
            target_path: String::from("$XDG_CONFIG_HOME"),
            respect_gitignore: true,
            ..Default::default()
        }
    }
}
