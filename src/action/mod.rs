mod types;
pub use types::ActionSource;

use anyhow::{Context, Result};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

use crate::{
    composition::{AppArgs, CompContext},
    config::{self, ConfigMap},
    model::Composition,
};

pub fn deploy(
    path: Option<String>,
    arg: Vec<String>,
    app_args: AppArgs,
    source: ActionSource,
) -> Result<()> {
    init_logger(app_args.is_verbose, app_args.is_debug);

    //loader
    //TODO: load to config_map directly
    let (base_path, config_sets) = config::load_configs(path.as_deref())?;
    let config_map = ConfigMap::new(base_path, &config_sets)?;

    match source {
        ActionSource::Render => {
            for comp_name in arg.iter() {
                let comp = config_map.get_comp(comp_name)?;
                deploy_comp(comp_name, comp, &config_map, app_args)?;
            }
        }
        ActionSource::Draw => {
            let comp = Composition::new(arg.clone());
            deploy_comp("draw_sketches", &comp, &config_map, app_args)?;
        }
    }
    Ok(())
}

fn deploy_comp(
    comp_name: &str,
    comp: &Composition,
    config_map: &ConfigMap<'_>,
    app_args: AppArgs,
) -> Result<()> {
    info!("Rendering Composition : {} ...", comp_name);
    let c = CompContext::new(comp_name, comp, config_map, &app_args)
        .context(format!("Composition init failed:{}", comp_name))?;

    c.deploy()
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
