mod loader;
mod map;
use crate::helpers::ResolvedPath;
use crate::model::DidmConfig;

pub use loader::*;
pub use map::ConfigMap;

pub struct ConfigSet(ResolvedPath, DidmConfig);

mod main_config;
pub use main_config::MainConfig;
//TODO: sort imports
