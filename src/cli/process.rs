use super::parser::{Cli, Commands};
use crate::action;
use crate::{composition::AppArgs, config};
use anyhow::{Ok, Result};
use clap::Parser;

//TODO: 1. Don't use draw, use render directly,turn draw to --sketch flag
//2.create an action builder
pub fn process() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Some(Commands::Init { path, .. }) => {
            config::init_config(path.as_deref())?;
        }
        Some(Commands::Render {
            comp_name,
            path,
            dry_run,
            verbose,
            tree,
            variants,
        }) => {
            //Porcess arg first,we may use in loader
            let app_args = AppArgs {
                variants,
                is_dryrun: dry_run,
                is_verbose: verbose,
                is_debug: args.debug,
                show_tree: tree,
            };
            action::deploy(path, comp_name, app_args, action::ActionSource::Render)?;
        }
        Some(Commands::Draw {
            sketch_names,
            path,
            dry_run,
            verbose,
            tree,
            variants,
        }) => {
            let app_args = AppArgs {
                variants,
                is_dryrun: dry_run,
                is_verbose: verbose,
                is_debug: args.debug,
                show_tree: tree,
            };
            action::deploy(path, sketch_names, app_args, action::ActionSource::Draw)?;
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
