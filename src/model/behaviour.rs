use ignore::overrides;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Behaviour {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_existed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_existed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_at_commands_error: Option<bool>,
}
impl Default for Behaviour {
    fn default() -> Self {
        Behaviour {
            overwrite_existed: Some(false),
            backup_existed: Some(true),
            stop_at_commands_error: Some(false),
        }
    }
}
//NOTE: should not use unwrap_or
//but for safe reason...
//TODO:should use getter
impl Behaviour {
    pub fn new(other: &Option<Behaviour>) -> Self {
        let me = Behaviour::default();
        me.override_by(other)
    }
    pub fn should_backup(&self) -> bool {
        self.backup_existed.unwrap_or(true) && self.overwrite_existed.unwrap_or(false)
    }
    //NOTE: overide by Some Directly
    //but still not good enough...
    //since this overried workflow is not common
    //
    pub fn override_by(&self, option_behaviour: &Option<Behaviour>) -> Self {
        match option_behaviour {
            Some(other) => Behaviour {
                overwrite_existed: other.overwrite_existed.or(self.overwrite_existed),
                backup_existed: other.backup_existed.or(self.backup_existed),
                stop_at_commands_error: other
                    .stop_at_commands_error
                    .or(self.stop_at_commands_error),
            },
            None => self.clone(),
        }
    }
}
