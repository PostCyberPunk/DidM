use super::parser::{Cli, Commands};
use crate::config;
use anyhow::Ok;
use clap::Parser;

pub fn process() -> anyhow::Result<()> {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Init { path, .. }) => {
            let config_path = path.clone().unwrap_or(String::from("./."));
            config::init_config(&config_path)?;
        }
        Some(Commands::Run { runner, path, .. }) => {
            let config_path = path.clone().unwrap_or(String::from("./."));
            let configs = config::load_configs(&config_path)?;
            todo!(
                "Run Runner:{0} with config path:{1}",
                runner,
                configs.first().unwrap().base_path.to_string_lossy()
            );
        }
        None => {
            let configs = config::load_configs(&config_path)?;
            todo!(
                "Start Tui with config path:{0}",
                configs.first().unwrap().base_path.to_string_lossy()
            );
        }
    }
    Ok(())
}
