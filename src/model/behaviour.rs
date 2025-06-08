use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Behaviour {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_existed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_existed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_symlink: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_at_commands_error: Option<bool>,
}
impl Default for Behaviour {
    fn default() -> Self {
        Behaviour {
            overwrite_existed: Some(false),
            backup_existed: Some(true),
            update_symlink: Some(true),
            stop_at_commands_error: Some(false),
        }
    }
}
impl Behaviour {
    pub fn patch(&self) -> Self {
        let other = &Behaviour::default();
        Behaviour {
            overwrite_existed: self.overwrite_existed.or(other.overwrite_existed),
            backup_existed: self.backup_existed.or(other.backup_existed),
            update_symlink: self.update_symlink.or(other.update_symlink),
            stop_at_commands_error: self.stop_at_commands_error.or(other.stop_at_commands_error),
        }
    }
    pub fn override_by(&self, other: &Behaviour) -> Self {
        Behaviour {
            overwrite_existed: other.overwrite_existed.or(self.overwrite_existed),
            backup_existed: other.backup_existed.or(self.backup_existed),
            update_symlink: other.update_symlink.or(self.update_symlink),
            stop_at_commands_error: other.stop_at_commands_error.or(self.stop_at_commands_error),
        }
    }
}
pub fn Meger(dad: &Option<Behaviour>, son: &Option<Behaviour>) -> Behaviour {
    match (dad, son) {
        (Some(dad), Some(son)) => dad.override_by(son),
        (Some(dad), None) => dad.clone(),
        (None, Some(son)) => son.clone(),
        (None, None) => Behaviour::default(),
    }
}
//TODO: is there a better way?
// pub fn is_default(b: &Behaviour) -> bool {
//     !b.overwrite_existed.unwrap()
//         && b.backup_existed
//         && b.update_symlink
//         && !b.stop_at_commands_error
// }
