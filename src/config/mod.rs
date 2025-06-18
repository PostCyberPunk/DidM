mod loader;
pub use loader::*;

mod map;
pub use map::ConfigMap;

mod main_config;
pub use main_config::MainConfig;

use crate::helpers::ResolvedPath;
use crate::model::DidmConfig;
pub struct ConfigSet(ResolvedPath, DidmConfig);
