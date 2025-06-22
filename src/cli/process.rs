use super::parser::{Cli, Commands};
use crate::config::ConfigMap;
use crate::{
    composition::{AppArgs, CompContext},
    config,
};
use anyhow::{Context, Ok, Result};
use clap::Parser;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

pub fn process() -> Result<()> {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Init { path, .. }) => {
            config::init_config(path.as_deref())?;
        }
        Some(Commands::Render {
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

            init_logger(app_args.is_verbose, args.debug);

            //loader
            //TODO: load to config_map directly
            let (base_path, config_sets) = config::load_configs(path.as_deref())?;
            let config_map = ConfigMap::new(base_path, &config_sets)?;

            //TODO: seprate steps, prepare , backup , apply
            deploy_comp(comp_name, app_args, config_map)?;
        }
        Some(Commands::Draw {
            sketch_names,
            path,
            dry_run,
            verbose,
        }) => {
            let app_args = AppArgs {
                is_dryrun: *dry_run,
                is_verbose: *verbose,
            };

            init_logger(app_args.is_verbose, args.debug);

            //loader
            //TODO: load to config_map directly
            let (base_path, config_sets) = config::load_configs(path.as_deref())?;
            let config_map = ConfigMap::new(base_path, &config_sets)?;

            //TODO: seprate steps, prepare , backup , apply
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

fn deploy_comp(comp_name: &String, app_args: AppArgs, config_map: ConfigMap<'_>) -> Result<()> {
    let comp = config_map.get_comp(comp_name)?;
    info!("Deploying Composition : {} ...", comp_name);

    CompContext::new(comp_name, comp, &config_map, &app_args)
        .context(format!("Composition init failed:{}", comp_name))?
        .deploy()
        .context(format!("Composition deploy failed:{}", comp_name))?;
    Ok(())
}

fn init_logger(is_verbose: bool, is_debug: bool) {
    //Prepare logger
    //tracing init
    let std_log_level = match (is_verbose, is_debug) {
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

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
