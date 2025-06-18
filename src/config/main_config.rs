use crate::model::CheckConfig;
use crate::model::DidmConfig;

use crate::model::Behaviour;

#[derive(Debug)]
pub struct MainConfig {
    pub(crate) behaviour: Behaviour,
    pub(crate) check_config: CheckConfig,
}
impl MainConfig {
    pub fn new(config: &DidmConfig) -> Self {
        let behaviour = Behaviour::new(&config.behaviour);
        let check_config = match config.skip_check {
            Some(c) => c,
            None => CheckConfig::new(),
        };
        MainConfig {
            behaviour,
            check_config,
        }
    }
}
