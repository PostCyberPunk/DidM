use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{Behaviour, CheckConfig, Composition, Sketch};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DidmConfig {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub behaviour: Option<Behaviour>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_check: Option<CheckConfig>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub sketch: HashMap<String, Sketch>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub composition: HashMap<String, Composition>,
}
impl DidmConfig {
    pub fn new() -> Self {
        DidmConfig {
            include: Vec::new(),
            skip_check: None,
            behaviour: None,
            sketch: HashMap::from([("basic".to_string(), Sketch::new())]),
            composition: HashMap::from([(
                "basic".to_string(),
                Composition {
                    sketch: vec!["basic".to_string()],
                    ..Default::default()
                },
            )]),
        }
    }
}
