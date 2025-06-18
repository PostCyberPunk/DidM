use super::{ConfigSet, MainConfig};
use crate::{
    helpers::{self, Helpers, ResolvedPath},
    model::{Behaviour, Plan, Profile},
};
use anyhow::Result;
use std::collections::HashMap;
use thiserror::Error;

//TODO: We should own everything in this map,
//convert back to normal configset when saving
//everything should be private
#[derive(Debug)]
pub struct ConfigMap<'a> {
    pub path_map: Vec<ResolvedPath>,
    pub main_config: MainConfig,
    pub profile_map: HashMap<&'a str, (usize, &'a Profile)>,
    pub plan_map: HashMap<&'a str, &'a Plan>,
    pub helpers: Helpers,
}
impl<'a> ConfigMap<'a> {
    pub fn new(base_path: ResolvedPath, config_sets: &'a [ConfigSet]) -> Result<Self> {
        let main_config = MainConfig::new(&config_sets[0].1);
        //---------Check Configs---------
        let check_config = &main_config.check_config;

        let helpers = helpers::Helpers::new(check_config);
        helpers
            .checker
            .working_dir_is_symlink(config_sets[0].0.get_raw())?;
        helpers.checker.is_git_workspace(base_path.get())?;

        //---------Build Config Map---------
        let mut path_map = Vec::new();
        let mut plan_map = HashMap::new();
        let mut profile_map = HashMap::new();

        for (idx, ConfigSet(config_path, config)) in config_sets.iter().enumerate() {
            //-------create config Path-----------------
            //NOTE: path in the config_sets is didm.toml,so we have to convert it
            let config_path = config_path.to_parent()?;
            path_map.push(config_path);

            for (name, plan) in &config.plans {
                if plan_map.contains_key(name.as_str()) {
                    return Err(ConfigError::DuplicatedPlan(name.to_string()).into());
                }
                plan_map.insert(name.as_str(), plan);
            }

            for (name, profile) in &config.profiles {
                if profile_map.contains_key(name.as_str()) {
                    return Err(ConfigError::DuplicatedProfile(name.to_string()).into());
                }
                profile_map.insert(name.as_str(), (idx, profile));
            }
        }
        if profile_map.is_empty() {
            return Err(ConfigError::NoProfileFound.into());
        }
        if plan_map.is_empty() {
            return Err(ConfigError::NoPlanFound.into());
        }

        //---------return Config Map---------
        Ok(ConfigMap {
            path_map,
            main_config,
            plan_map,
            profile_map,
            helpers,
        })
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
    pub fn get_main_behaviour(&self) -> &Behaviour {
        &self.main_config.behaviour
    }
    pub fn get_helpers(&self) -> &Helpers {
        &self.helpers
    }
    pub fn get_base_path(&self, idx: usize) -> Result<&ResolvedPath> {
        if idx > self.path_map.len() {
            return Err(
                ConfigError::IndexOutbound(String::from("PathMap"), idx.to_string()).into(),
            );
        }
        Ok(&self.path_map[idx])
    }
    pub fn get_main_base_path(&self) -> Result<&ResolvedPath> {
        self.get_base_path(0)
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Config file already existed in {0}")]
    ConfigExists(PathBuf),

    #[error("Can't find any plan,check your config,maybe there is a typo?")]
    NoPlanFound,

    #[error("Can't find any profile,check your config,maybe there is a typo?")]
    NoProfileFound,

    #[error("Plan `{0}` is duplicated")]
    DuplicatedPlan(String),

    #[error("Profile `{0}` is duplicated")]
    DuplicatedProfile(String),

    #[error("Plan {0}  not found.")]
    PlanNotFound(String),

    #[error("Profile `{0}` not found.")]
    ProfileNotFound(String),

    #[error("Index of `{0}` is out of bound: `{1}`")]
    IndexOutbound(String, String),

    #[error("Can't find any plan,check your config,maybe there is a typo?")]
    NoPlanFound,

    #[error("Can't find any profile,check your config,maybe there is a typo?")]
    NoProfileFound,
}
