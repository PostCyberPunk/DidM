use super::parser::{Cli, Commands};
use crate::config::ConfigMap;
use crate::log::LogLevel;
use crate::utils;
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
            //Porcess arg first,we may use in loader
            let plan_args = PlanArgs {
                is_dryrun: *dry_run,
                is_verbose: *verbose,
            };
            //TODO:File logger
            //Prepare logger, we may use in loaer too
            //FIX: File logger need flush ,but error will cause it never flush
            let mut logger = Logger::new();
            let std_log_level = match (plan_args.is_verbose, args.debug) {
                (_, true) => LogLevel::Debug,
                (true, false) => LogLevel::Info,
                (false, false) => LogLevel::Warn,
            };
            logger.add_target(StdoutLogTarget::new(std_log_level));

            //loader
            //TODO: load to config_map directly
            let (base_path, config_sets) = config::load_configs(path.as_deref())?;
            let config_map = ConfigMap::new(base_path, &config_sets)?;

            //TODO: seprate steps, prepare , backup , apply
            PlanContext::new(plan_name, &config_map, &plan_args, &logger)
                .context(format!("Plan init failed:{}", plan_name))?
                .deploy()
                .context(format!("Plan deploy failed:{}", plan_name))?;
        }
        Some(Commands::Schema) => {
            let path = std::env::current_dir().unwrap().join("didm.schema.json");
            if path.exists() && !crate::utils::prompt::confirm("File exists,overwrite?") {
                return Err(anyhow::anyhow!("User cancelled"));
            }
            let schema = schemars::schema_for!(crate::model::DidmConfig);
            std::fs::write(path.clone(), serde_json::to_string_pretty(&schema).unwrap())?;
            println!("schema generated:{}", path.display());
        }
        None => {
            // let configs = config::load_configs(args.path.as_deref())?;
        }
    }
    Ok(())
}
