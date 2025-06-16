use crate::model::SkipCheck;

use crate::model::Behaviour;

pub struct MainConfig {
    pub(crate) behaviour: Behaviour,
    pub(crate) skipcheck: SkipCheck,
}
