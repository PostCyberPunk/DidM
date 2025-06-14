use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Behaviour {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_existed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_existed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_symlink: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_at_commands_error: Option<bool>,
}
impl Default for Behaviour {
    fn default() -> Self {
        Behaviour {
            overwrite_existed: Some(false),
            backup_existed: Some(true),
            backup_symlink: Some(false),
            stop_at_commands_error: Some(false),
        }
    }
}
//NOTE: should not use unwrap_or
//but for safe reason...
//TODO:should use getter
impl Behaviour {
    pub fn should_backup(&self) -> bool {
        self.backup_existed.unwrap_or(true) && self.overwrite_existed.unwrap_or(false)
    }

    pub fn override_by(&self, other: &Behaviour) -> Self {
        Behaviour {
            overwrite_existed: other.overwrite_existed.or(self.overwrite_existed),
            backup_existed: other.backup_existed.or(self.backup_existed),
            backup_symlink: other.backup_symlink.or(self.backup_symlink),
            stop_at_commands_error: other.stop_at_commands_error.or(self.stop_at_commands_error),
        }
    }
}
//TODO: we need a better name
pub fn Meger(dad: &Option<Behaviour>, son: &Option<Behaviour>) -> Behaviour {
    match (dad, son) {
        (Some(dad), Some(son)) => dad.override_by(son),
        (Some(dad), None) => dad.clone(),
        (None, Some(son)) => son.clone(),
        (None, None) => Behaviour::default(),
    }
}
