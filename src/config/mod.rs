mod loader;
pub use loader::*;

mod map;
pub use map::ConfigMap;

mod main_config;
pub use main_config::MainConfig;
use once_cell::sync::OnceCell;

use crate::model::{CheckConfig, DidmConfig};
use crate::utils::ResolvedPath;
pub struct ConfigSet(ResolvedPath, DidmConfig);

pub static CHCECK_CONFIG: OnceCell<CheckConfig> = OnceCell::new();
