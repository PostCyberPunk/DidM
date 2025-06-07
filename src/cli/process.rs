use super::parser::{Cli, Commands};
use crate::config;
use anyhow::Ok;
use clap::Parser;

pub fn process() -> anyhow::Result<()> {
    let args = Cli::parse();

    let config_path = args.path.unwrap_or(String::from("./."));
    match &args.command {
        Some(Commands::Init { .. }) => {
            config::init_config(&config_path)?;
        }
        Some(Commands::Run { runner, .. }) => {
            config::load_config(&config_path)?;
            todo!("Run Runner:{0} with config path:{1}", runner, config_path);
        }
        None => {
            config::load_config(&config_path)?;
            todo!("Start Tui")
        }
    }
    Ok(())
}
