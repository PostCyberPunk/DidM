mod types;
use std::result;

pub use types::ActionSource;

use anyhow::{Context, Result};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

use crate::{
    composition::{AppArgs, CompContext},
    config::{self, ConfigMap},
    entries::TreeManager,
    model::{Composition, sketch},
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

    //TODO: 1.add sketch_temp_comp to configs?
    //2. use a builder instead of comp_ctx to get entries_manager and commands_runner
    #[allow(unused_assignments)]
    let mut sketch_comp: Option<Composition> = None;

    let comps: Vec<(&Composition, String)> = match source {
        ActionSource::Render => {
            let mut comps = Vec::new();
            for comp_name in arg {
                let comp = config_map.get_comp(&comp_name)?;
                comps.push((comp, comp_name));
            }
            comps
        }
        ActionSource::Draw => {
            let comp = Composition::new(arg);
            let comp_name = String::from("draw_sketches");
            sketch_comp = Some(comp);
            vec![(sketch_comp.as_ref().unwrap(), comp_name)]
        }
    };

    //Preparing comp_ctxs
    let mut comp_ctxs: Vec<(CompContext, &str)> = Vec::new();
    for (comp, comp_name) in comps.iter() {
        info!("Preparing Composition : {} ...", comp_name);
        let ctx = CompContext::new(comp_name, comp, &config_map, &app_args)
            .context(format!("Composition init failed:{}", comp_name))?;
        comp_ctxs.push((ctx, comp_name));
    }

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
