mod loader;
mod map;
use crate::helpers::ResolvedPath;
use crate::model::DidmConfig;

pub use loader::*;
pub use map::ConfigMap;

pub struct ConfigSet(ResolvedPath, DidmConfig);
