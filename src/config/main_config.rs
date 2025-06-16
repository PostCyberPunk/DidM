use crate::model::CheckConfig;
use crate::model::DidmConfig;

use crate::model::Behaviour;

pub struct MainConfig {
    pub(crate) behaviour: Behaviour,
    pub(crate) skipcheck: CheckConfig,
}
impl MainConfig {
    pub fn new(config: &DidmConfig) -> Self {
        let behaviour = Behaviour::new(&config.behaviour);
        let skipcheck = match config.skip_check {
            Some(c) => c,
            None => CheckConfig::new(),
        };
        MainConfig {
            behaviour,
            skipcheck,
        }
    }
}
