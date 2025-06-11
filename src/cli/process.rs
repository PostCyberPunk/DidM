use super::parser::{Cli, Commands};
use crate::log::LogLevel;
use crate::plan;
use crate::{
    config,
    log::{Logger, StdoutLogTarget, logger},
    model::Plan,
    plan::{PlanArgs, PlanContext},
};
use anyhow::{Context, Ok};
use clap::Parser;

pub fn process() -> anyhow::Result<()> {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Init { path, .. }) => {
            let config_path = path.clone().unwrap_or(String::from("./."));
            config::init_config(&config_path)?;
        }
        Some(Commands::Deploy {
            plan_name,
            path,
            dry_run,
            verbose,
        }) => {
            let config_path = path.clone().unwrap_or(String::from("./."));
            let configs = config::load_configs(&config_path)?;
            let plan_args = PlanArgs {
                is_dry_run: *dry_run,
                is_verbose: *verbose,
            };
            let mut logger = Logger::new();
            let std_level = if *verbose {
                LogLevel::Info
            } else {
                LogLevel::Warn
            };
            logger.add_target(StdoutLogTarget::new(std_level));
            PlanContext::new(plan_name, &configs, &plan_args, &logger)
                .context(format!("Plan:{}", plan_name))?
                .deploy()?;
        }
        None => {
            let config_path = args.path.unwrap_or(String::from("./."));
            let configs = config::load_configs(&config_path)?;
        }
    }
    Ok(())
}
