use crate::model::DidmConfig;
use crate::utils::{PathResolver, ResolvedPath};
use anyhow::{Context, Result};
use tracing::info;

use super::ConfigSet;
use super::map::ConfigError;
use std::fs;

const CONFIG_FILE_NAME: &str = "didm.toml";
const DEFAULT_PATH: &str = ".";
const DEFAULT_CONFIG_PATH: &str = "./didm.toml";

//TODO: add detailed error handling for load config
pub fn load_config(config_path: ResolvedPath) -> Result<ConfigSet> {
    let path = config_path.as_path();
    let content = fs::read_to_string(path)?;
    let config: DidmConfig = toml::from_str(&content)?;
    info!("loading from config:{path:#?}");
    Ok(ConfigSet(config_path, config))
}
pub fn load_configs(path: Option<&str>) -> Result<(ResolvedPath, Vec<ConfigSet>)> {
    //TODO: impl $DIDM_DEFAULT_CONFIG env
    let path = path.unwrap_or(DEFAULT_CONFIG_PATH);
    //TODO: map this error to Hint
    let resolved_config_path = PathResolver::resolve(path,true).with_context(|| {
            "Config file not found by current path,consider use `didm init` or specify path with `--path`".to_string()
        })?;

    info!("Loading configs from:{path}");
    let base_path = resolved_config_path.to_parent().unwrap();
    let base_configset = load_config(resolved_config_path)?;

    let mut config_sets = Vec::new();

    for p in base_configset.1.include.iter() {
        let _resolved_path = PathResolver::resolve_from(&base_path, p.as_str(), true)?;
        let s = load_config(_resolved_path)?;
        config_sets.push(s);
    }
    config_sets.insert(0, base_configset);
    Ok((base_path, config_sets))
}

//TODO: Save multiple configs
pub fn save_config(set: &ConfigSet) -> Result<()> {
    let ConfigSet(config_path, config) = set;
    let content = toml::to_string_pretty(config)?;
    fs::write(config_path.as_path(), content)?;
    Ok(())
}

pub fn init_config(path: Option<&str>) -> Result<()> {
    let path = path.unwrap_or(DEFAULT_PATH);

    let resolved_path = PathResolver::resolve(path, false)?;

    let config_path = resolved_path.into_child(CONFIG_FILE_NAME, false)?;
    if config_path.as_path().exists() {
        return Err(ConfigError::ConfigExists(config_path.into_pathbuf()).into());
    }
    let config = DidmConfig::new();
    let cfgset = ConfigSet(config_path, config);
    save_config(&cfgset)
}
