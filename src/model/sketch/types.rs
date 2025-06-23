use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, JsonSchema)]
pub struct ExtraEntry {
    pub source_path: String,
    pub target_path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum Mode {
    #[default]
    Symlink,
    Copy,
}
impl Mode {
    pub fn is_default(&self) -> bool {
        *self == Mode::default()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum Unit {
    #[default]
    File,
    Dir,
}
impl Unit {
    pub fn is_default(&self) -> bool {
        *self == Unit::default()
    }
}
