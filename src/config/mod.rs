use crate::model::DidmConfig;
use crate::path::PathHandler;
use anyhow::Result;
use std::fs;

pub const CONFIG_FILE_NAME: &str = "didm.toml";

pub fn load_config(path: &str) -> Result<DidmConfig> {
    let config_path = PathHandler::new(path).find_file(CONFIG_FILE_NAME)?;
    let content = fs::read_to_string(&config_path)?;
    let mut config: DidmConfig = toml::from_str(&content)?;
    config.base_path = config_path;
    Ok(config)
}
pub fn load_configs(path: &str) -> Result<Vec<DidmConfig>> {
    let base_config = load_config(path)?;
    let mut result = base_config
        .include
        .iter()
        .map(|path| load_config(path))
        .collect::<Result<Vec<DidmConfig>, _>>()?;
    result.insert(0, base_config);
    Ok(result)
}

pub fn save_config(cfg: &DidmConfig) -> Result<()> {
    let content = toml::to_string_pretty(cfg)?;
    fs::write(&cfg.base_path, content)?;
    Ok(())
}

pub fn init_config(path: &str) -> Result<()> {
    let config_path = PathHandler::new(path).find_file_or_ok(CONFIG_FILE_NAME)?;
    let cfg = DidmConfig::new(config_path);
    save_config(&cfg)
}
