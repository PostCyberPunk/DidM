use crate::model::DidmConfig;
use crate::model::SkipCheck;

use crate::model::Behaviour;

pub struct MainConfig {
    pub(crate) behaviour: Behaviour,
    pub(crate) skipcheck: SkipCheck,
}
impl MainConfig {
    pub fn new(config: &DidmConfig) -> Self {
        let behaviour = Behaviour::new(&config.behaviour);
        let skipcheck = match config.skip_check {
            Some(c) => c,
            None => SkipCheck::new(),
        };
        MainConfig {
            behaviour,
            skipcheck,
        }
    }
}
