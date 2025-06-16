use super::{Behaviour, Plan, Profile, SkipCheck};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DidmConfig {
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
    pub fn new() -> Self {
        DidmConfig {
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
