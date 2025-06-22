use super::parser::{Cli, Commands};
use crate::config::ConfigMap;
use crate::{
    composition::{AppArgs, CompContext},
    config,
};
use anyhow::{Context, Ok};
use clap::Parser;
use tracing_subscriber::FmtSubscriber;

pub fn process() -> anyhow::Result<()> {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Init { path, .. }) => {
            config::init_config(path.as_deref())?;
        }
        Some(Commands::Deploy {
            comp_name,
            path,
            dry_run,
            verbose,
        }) => {
            //Porcess arg first,we may use in loader
            let app_args = AppArgs {
                is_dryrun: *dry_run,
                is_verbose: *verbose,
            };

            //Prepare logger
            //tracing init
            let std_log_level = match (app_args.is_verbose, args.debug) {
                (_, true) => tracing::Level::DEBUG,
                (true, false) => tracing::Level::INFO,
                (false, false) => tracing::Level::WARN,
            };
            let subscriber = FmtSubscriber::builder()
                .pretty()
                .without_time()
                .with_ansi(true)
                .with_line_number(false)
                .with_file(false)
                .with_target(false)
                .compact()
                .with_max_level(std_log_level)
                .finish();

            tracing::subscriber::set_global_default(subscriber)
                .expect("setting default subscriber failed");

            //loader
            //TODO: load to config_map directly
            let (base_path, config_sets) = config::load_configs(path.as_deref())?;
            let config_map = ConfigMap::new(base_path, &config_sets)?;

            //TODO: seprate steps, prepare , backup , apply
            CompContext::new(comp_name, &config_map, &app_args)
                .context(format!("Composition init failed:{}", comp_name))?
                .deploy()
                .context(format!("Composition deploy failed:{}", comp_name))?;
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
