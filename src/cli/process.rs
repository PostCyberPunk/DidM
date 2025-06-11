use super::parser::{Cli, Commands};
use crate::log::LogLevel;
use crate::{
    config,
    log::{Logger, StdoutLogTarget},
    plan::{PlanArgs, PlanContext},
};
use anyhow::{Context, Ok};
use clap::Parser;

pub fn process() -> anyhow::Result<()> {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Init { path, .. }) => {
            config::init_config(path.as_deref())?;
        }
        Some(Commands::Deploy {
            plan_name,
            path,
            dry_run,
            verbose,
        }) => {
            let configs = config::load_configs(path.as_deref())?;
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
            let configs = config::load_configs(args.path.as_deref())?;
        }
    }
    Ok(())
}
