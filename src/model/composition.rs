use std::collections::HashMap;

use crate::model::behaviour::Behaviour;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, JsonSchema)]
pub struct Composition {
    pub sketch: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_build_commands: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub post_build_commands: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commands_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub override_behaviour: Option<Behaviour>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub environment: HashMap<String, String>,
}
impl Composition {
    pub fn new(sketches: Vec<String>) -> Self {
        Self {
            sketch: sketches,
            ..Default::default()
        }
    }
}
