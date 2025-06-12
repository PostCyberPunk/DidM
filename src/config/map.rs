use crate::model::{DidmConfig, Plan, Profile};
use anyhow::Result;
use std::collections::HashMap;
use thiserror::Error;

//TODO: consiser use Instance for config map
pub struct ConfigMap<'a> {
    pub main_config: &'a DidmConfig,
    pub configs: &'a [DidmConfig],
    pub profile_map: HashMap<&'a str, (usize, &'a Profile)>,
    pub plan_map: HashMap<&'a str, &'a Plan>,
}
impl<'a> ConfigMap<'a> {
    pub fn new(configs: &'a [DidmConfig]) -> Self {
        let main_config = &configs[0];

        let mut plan_map = HashMap::new();
        let mut profile_map = HashMap::new();

        //TODO: Validate duplicated keys
        for (idx, config) in configs.iter().enumerate() {
            plan_map.extend(
                config
                    .plans
                    .iter()
                    .map(|(name, plan)| (name.as_str(), plan)),
            );
            profile_map.extend(
                config
                    .profiles
                    .iter()
                    .map(|(name, profile)| (name.as_str(), (idx, profile))),
            );
        }

        ConfigMap {
            main_config,
            configs,
            plan_map,
            profile_map,
        }
    }
    pub fn get_plan(&self, plan_name: &str) -> Result<&Plan> {
        //NOTE: use match to avoid deref of a ref...
        match self.plan_map.get(plan_name) {
            Some(plan) => Ok(plan),
            None => Err(ConfigError::PlanNotFound(plan_name.to_string()).into()),
        }
    }
    pub fn get_profile(&self, profile_name: &'a str) -> Result<(&Profile, usize, &'a str)> {
        match self.profile_map.get(profile_name) {
            Some((idx, profile)) => Ok((profile, *idx, profile_name)),
            None => Err(ConfigError::ProfileNotFound(profile_name.to_string()).into()),
        }
    }
    pub fn get_profiles(
        &'a self,
        profiles: &'a [String],
    ) -> Result<Vec<(&'a Profile, usize, &'a str)>> {
        let mut result = Vec::new();
        for profile_name in profiles {
            let p = self.get_profile(profile_name)?;
            result.push(p);
        }
        Ok(result)
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Plan {0}  not found.")]
    PlanNotFound(String),

    #[error("Profile `{0}` not found.")]
    ProfileNotFound(String),
}
