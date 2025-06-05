use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Behaviour {
    #[serde(default, skip_serializing_if = "super::is_false")]
    pub overwrite_existed: bool,
    #[serde(
        default = "super::default_true",
        skip_serializing_if = "super::is_true"
    )]
    pub backup_existed: bool,
    #[serde(
        default = "super::default_true",
        skip_serializing_if = "super::is_true"
    )]
    pub update_symlink: bool,
    #[serde(default, skip_serializing_if = "super::is_false")]
    pub stop_at_commands_error: bool,
}
impl Default for Behaviour {
    fn default() -> Self {
        Behaviour {
            overwrite_existed: false,
            backup_existed: true,
            update_symlink: true,
            stop_at_commands_error: false,
        }
    }
}
//TODO: is there a better way?
pub fn is_default(b: &Behaviour) -> bool {
    !b.overwrite_existed && b.backup_existed && b.update_symlink && !b.stop_at_commands_error
}
