use super::args::AppArgs;
use crate::{
    commands::{CommandsContext, CommandsRunner},
    config::ConfigMap,
    entries::EntriesManager,
    log::Logger,
    model::Sketch,
    utils::PathResolver,
};
use anyhow::{Context, Result};

pub struct CompContext<'a> {
    pub commands_runner: CommandsRunner<'a>,
    pub entries_manager: EntriesManager<'a>,
}

impl<'a> CompContext<'a> {
    pub fn new(
        comp_name: &'a str,
        config_map: &'a ConfigMap,
        args: &'a AppArgs,
        logger: &'a Logger,
    ) -> Result<Self> {
        //NOTE: order should be: error with less calculation ; then error with lager calulation
        logger.info(&format!("Deploying Composition : {} ...", comp_name));

        let base_path = config_map.get_main_base_path()?;
        let comp = config_map.get_comp(comp_name)?;

        let mut commands_runner = CommandsRunner::new(logger, args.is_dryrun);
        let mut all_entries = EntriesManager::new(logger, args.is_dryrun);

        //Get Bhaviour
        let behaviour = config_map
            .get_main_behaviour()
            .override_by(&comp.override_behaviour);

        //Prepare Command runner
        let envrironment = &comp.environment;
        let stop_at_commands_error = behaviour.stop_at_commands_error.unwrap();
        let commands_path =
            PathResolver::resolve_from_or_base(base_path, &comp.commands_path)?.into_pathbuf();
        let comp_cmd_ctx = CommandsContext::new(
            envrironment,
            commands_path,
            stop_at_commands_error,
            &comp.pre_build_commands,
            &comp.post_build_commands,
        );
        commands_runner.add_context(comp_cmd_ctx);

        //apply sketchs
        let sketchs = config_map.get_sketches(&comp.sketch)?;
        for tuple in sketchs {
            logger.info(&format!("Preparing sketch: {}", tuple.2));
            Self::collect_sketch(
                config_map,
                &mut commands_runner,
                &mut all_entries,
                behaviour,
                tuple,
            )
            .context(format!("Sketch: {}", tuple.2))?;
        }

        Ok(CompContext {
            // sketch_ctxs,
            commands_runner,
            entries_manager: all_entries,
        })
    }
    pub fn deploy(self) -> Result<()> {
        // let mut backuper = Backuper::init(self.base_path, self.name.to_string(), args.is_dry_run)?;
        self.commands_runner.run_pre_commands()?;

        self.entries_manager.copy_and_link()?;

        self.commands_runner.run_post_commands()?;
        Ok(())
    }
    fn collect_sketch(
        config_map: &'a ConfigMap<'_>,
        commands_runner: &mut CommandsRunner<'a>,
        entries_manager: &mut EntriesManager<'a>,
        behaviour: crate::model::Behaviour,
        tuple: (&'a Sketch, usize, &str),
    ) -> Result<(), anyhow::Error> {
        let (sketch, idx, sketch_name) = tuple;

        let base_path = config_map.get_base_path(idx)?;
        let behaviour = behaviour.override_by(&sketch.override_behaviour);

        let stop_at_commands_error = behaviour.stop_at_commands_error.unwrap();
        commands_runner.add_sketch_context(sketch, base_path, stop_at_commands_error)?;

        entries_manager.add_sketch(sketch, base_path, &behaviour, sketch_name)?;
        Ok(())
    }
}
