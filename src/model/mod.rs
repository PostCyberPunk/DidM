pub mod behaviour;
pub mod plan;
pub mod profile;
mod skip_check;

pub use behaviour::Behaviour;
pub use plan::Plan;
pub use profile::Profile;
pub use skip_check::SkipCheck;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DidmConfig {
    #[serde(skip)]
    pub base_path: PathBuf,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub behaviour: Option<Behaviour>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_check: Option<SkipCheck>,
    pub profiles: HashMap<String, Profile>,
    pub plans: HashMap<String, Plan>,
}
impl DidmConfig {
    pub fn new(base_path: PathBuf) -> Self {
        DidmConfig {
            base_path,
            include: Vec::new(),
            skip_check: None,
            behaviour: None,
            profiles: HashMap::from([("basic".to_string(), Profile::new())]),
            plans: HashMap::from([(
                "basic".to_string(),
                Plan {
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
