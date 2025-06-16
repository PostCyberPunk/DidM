use crate::helpers::{PathResolver, ResolvedPath};
use crate::model::DidmConfig;
use anyhow::{Context, Result};

use super::ConfigSet;
use std::fs;

const CONFIG_FILE_NAME: &str = "didm.toml";
const DEFAULT_PATH: &str = ".";
const DEFAULT_CONFIG_PATH: &str = "./didm.toml";

//TODO: add detailed error handling for load config
pub fn load_config(config_path: ResolvedPath) -> Result<ConfigSet> {
    let content = fs::read_to_string(&config_path.get())?;
    let config: DidmConfig = toml::from_str(&content)?;
    Ok(ConfigSet(config_path, config))
}
pub fn load_configs(path: Option<&str>) -> Result<(ResolvedPath, Vec<ConfigSet>)> {
    let path = path.unwrap_or(DEFAULT_CONFIG_PATH);
    let resolver = &PathResolver::new(true);
    //TODO: map this error to Hint
    let resolved_config_path = resolver.resolve(path).with_context(|| {
        format!(
            "Config file not found in current path,consider use `didm init` or specify path with `--path`"
        )})?;

    let base_path = resolved_config_path.to_parent().unwrap();
    let base_configset = load_config(resolved_config_path)?;

    let mut config_sets = Vec::new();

    for p in base_configset.1.include.iter() {
        let _resolved_path = resolver.resolve_from(&base_path, p.as_str())?;
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
    fs::write(&config_path.get(), content)?;
    Ok(())
}

pub fn init_config(path: Option<&str>) -> Result<()> {
    let path = path.unwrap_or(DEFAULT_PATH);
    let resolver = PathResolver::new(true);

    let resolved_path = resolver.resolve(path)?;

    let config_path = resolved_path.into_child(CONFIG_FILE_NAME)?;
    let config = DidmConfig::new();
    let cfgset = ConfigSet(config_path, config);
    save_config(&cfgset)
}
