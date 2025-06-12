mod map;
use crate::model::DidmConfig;
use crate::path::PathBufExtension;
use anyhow::Result;
pub use map::ConfigMap;
use std::fs;
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = "didm.toml";
const DEFAULT_PATH: &str = ".";
const DEFAULT_CONFIG_PATH: &str = "./didm.toml";
//TODO: add validation for config,ie. check duplicated profiles
//
//TODO: add detailed error handling for load config

pub fn load_config(path: &str) -> Result<DidmConfig> {
    let config_path = PathBuf::from(path).resolve()?;
    if !config_path.exists() {
        return Err(anyhow::anyhow!(
            "Config file not found: {}",
            config_path.display()
        ));
    }
    let content = fs::read_to_string(&config_path)?;
    let mut config: DidmConfig = toml::from_str(&content)?;
    config.base_path = config_path.parent().unwrap().to_path_buf();
    Ok(config)
}
pub fn load_configs(path: Option<&str>) -> Result<Vec<DidmConfig>> {
    let path = path.unwrap_or(DEFAULT_CONFIG_PATH);
    if !PathBuf::from(path).exists() {
        return Err(anyhow::anyhow!(
            "Config file not found in current path,consider use `didm init` or specify path with `--path`"
        ));
    }
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
    let path = path.unwrap_or(DEFAULT_PATH);
    let config_path = PathBuf::from(path).find_file_or_ok(CONFIG_FILE_NAME)?;
    let cfg = DidmConfig::new(config_path);
    save_config(&cfg)
}
