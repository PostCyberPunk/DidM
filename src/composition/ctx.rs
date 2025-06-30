///?
use super::args::AppArgs;
use crate::{
    bakcup::{BackupManager, BackupRoot},
    commands::{CommandsContext, CommandsRunner},
    config::ConfigMap,
    entries::{EntriesManager, EntryCollector, TreeManager},
    model::{Composition, Sketch},
    utils::PathResolver,
};
use anyhow::Result;

//NOTE:
//1.this name is not accurate .
//2.we can use this to create preview, but we don't need commands_runner,so we need more
//  abstraction here,but they do share a few things in comp(base_path,behaviour)
//3.since collect commands is not that expansive...we can keep this for now
pub struct CompContext<'a> {
    commands_runner: CommandsRunner<'a>,
    entries_manager: EntriesManager,
    // runtime: &'a Runtime,
}

impl<'a> CompContext<'a> {
    pub fn new(
        comp_name: &'a str,
        comp: &'a Composition,
        config_map: &'a ConfigMap<'_>,
        args: &'a AppArgs,
    ) -> Result<Self> {
        //NOTE: order should be: error with less calculation ; then error with more calulation
        //
        //FIX:!!!!!!!!!this name is unclear
        //we should rename it to something like main_path
        let base_path = config_map.get_main_base_path()?;

        let mut commands_runner = CommandsRunner::new(args.is_dryrun);
        let mut entries_manager = EntriesManager::new(args.is_dryrun);

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

        //prepare backup root
        let backup_root = BackupRoot::new(base_path, comp_name, args.is_dryrun)?;
        //apply sketchs
        let sketchs = config_map.get_sketches(&comp.sketch)?;

        //TODO:we need some kind of summary of result here
        for tuple in sketchs {
            let result = Self::collect_sketch(
                config_map,
                &mut commands_runner,
                &mut entries_manager,
                &backup_root,
                behaviour,
                tuple,
            );
            if let Err(e) = result {
                return Err(anyhow::anyhow!("Sketch {} failed: {}", tuple.2, e));
            }
        }
        //is backup created?
        backup_root.has_bakcup();

        Ok(CompContext {
            commands_runner,
            entries_manager,
            // runtime,
        })
    }
    pub fn fill_tree(&self, tree: &mut TreeManager) {
        self.entries_manager.fill_tree(tree);
    }
    pub fn deploy(self) -> Result<()> {
        // let mut backuper = Backuper::init(self.base_path, self.name.to_string(), args.is_dry_run)?;
        self.commands_runner.run_pre_commands()?;

        self.entries_manager.apply_all();

        self.commands_runner.run_post_commands()?;
        Ok(())
    }
    fn collect_sketch(
        config_map: &'a ConfigMap<'_>,
        commands_runner: &mut CommandsRunner<'a>,
        entries_manager: &mut EntriesManager,
        backup_root: &BackupRoot,
        behaviour: crate::model::Behaviour,
        tuple: (&'a Sketch, usize, &str),
    ) -> Result<()> {
        let (sketch, idx, sketch_name) = tuple;

        let base_path = config_map.get_base_path(idx)?;
        let behaviour = behaviour.override_by(&sketch.override_behaviour);

        let stop_at_commands_error = behaviour.stop_at_commands_error.unwrap();
        commands_runner.add_sketch_context(sketch, base_path, stop_at_commands_error)?;

        let bakcuper = match behaviour.should_backup() {
            false => None,
            true => Some(BackupManager::init(
                backup_root,
                format!("sketch_{}", sketch_name),
            )?),
        };
        EntryCollector::new(
            entries_manager,
            sketch,
            base_path,
            sketch_name,
            &behaviour,
            bakcuper.as_ref(),
        )?
        .collect()?;

        // entries_manager.add_sketch(sketch, base_path, &behaviour, sketch_name)?;
        Ok(())
    }
}
