pub mod behaviour;
pub mod profile;
pub mod runner;

pub use behaviour::Behaviour;
pub use profile::Profile;
pub use runner::Runner;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DidmConfig {
    #[serde(skip)]
    pub base_path: PathBuf,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<String>,
    #[serde(default, skip_serializing_if = "behaviour::is_default")]
    pub behaviour: Behaviour,
    pub profiles: HashMap<String, Profile>,
    pub runners: HashMap<String, Runner>,
}
impl DidmConfig {
    pub fn new(base_path: PathBuf) -> Self {
        DidmConfig {
            base_path,
            include: Vec::new(),
            behaviour: Behaviour::default(),
            profiles: HashMap::from([("basic".to_string(), Profile::new())]),
            runners: HashMap::from([(
                "basic".to_string(),
                Runner {
                    profiles: vec!["basic".to_string()],
                    ..Default::default()
                },
            )]),
        }
    }
}
fn is_true(val: &bool) -> bool {
    *val
}
fn default_true() -> bool {
    true
}
fn is_false(val: &bool) -> bool {
    !*val
}
