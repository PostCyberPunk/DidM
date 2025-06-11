use crate::model::DidmConfig;
use crate::path::RawPathHandler;
use anyhow::Result;
use std::fs;

pub const CONFIG_FILE_NAME: &str = "didm.toml";

//TODO: add validation for config,ie. check duplicated profiles
//
//TODO: add detailed error handling for load config

pub fn load_config(path: Option<&str>) -> Result<DidmConfig> {
    let config_path = RawPathHandler::new(path)
        .resolve()?
        .find_file(CONFIG_FILE_NAME)?;
    let content = fs::read_to_string(&config_path)?;
    let mut config: DidmConfig = toml::from_str(&content)?;
    config.base_path = config_path;
    Ok(config)
}
pub fn load_configs(path: Option<&str>) -> Result<Vec<DidmConfig>> {
    let base_config = load_config(path)?;
    let mut result = base_config
        .include
        .iter()
        .map(|path| load_config(path))
        .collect::<Result<Vec<DidmConfig>, _>>()?;
    result.insert(0, base_config);
    Ok(result)
}

//TODO: Save multiple configs
pub fn save_config(cfg: &DidmConfig) -> Result<()> {
    let content = toml::to_string_pretty(cfg)?;
    fs::write(&cfg.base_path, content)?;
    Ok(())
}

pub fn init_config(path: Option<&str>) -> Result<()> {
    let config_path = RawPathHandler::new(path)
        .resolve()?
        .find_file_or_ok(CONFIG_FILE_NAME)?;
    let cfg = DidmConfig::new(config_path);
    save_config(&cfg)
}
