use super::args::PlanArgs;
use crate::{
    commands::{CommandsContext, CommandsRunner},
    config::ConfigMap,
    entries::AllEntries,
    log::Logger,
    model::Sketch,
    utils::PathResolver,
};
use anyhow::{Context, Result};

pub struct PlanContext<'a> {
    pub commands_runner: CommandsRunner<'a>,
    pub all_entries: AllEntries<'a>,
}

impl<'a> PlanContext<'a> {
    pub fn new(
        plan_name: &'a str,
        config_map: &'a ConfigMap,
        args: &'a PlanArgs,
        logger: &'a Logger,
    ) -> Result<Self> {
        //NOTE: order should be: error with less calculation ; then error with lager calulation
        logger.info(&format!("Deploying plan : {} ...", plan_name));

        let base_path = config_map.get_main_base_path()?;
        // (&plan.commands_path)?;
        let plan = config_map.get_plan(plan_name)?;

        let mut commands_runner = CommandsRunner::new(logger, args.is_dryrun);
        let mut all_entries = AllEntries::new(logger, args.is_dryrun);

        //Get Bhaviour
        let behaviour = config_map
            .get_main_behaviour()
            .override_by(&plan.override_behaviour);

        //Prepare Command runner
        let envrironment = &plan.environment;
        let stop_at_commands_error = behaviour.stop_at_commands_error.unwrap();
        let commands_path =
            PathResolver::resolve_from_or_base(base_path, &plan.commands_path)?.into_pathbuf();
        let plan_cmd_ctx = CommandsContext::new(
            envrironment,
            commands_path,
            stop_at_commands_error,
            &plan.pre_build_commands,
            &plan.post_build_commands,
        );
        commands_runner.add_context(plan_cmd_ctx);

        //apply profiles
        let profiles = config_map.get_profiles(&plan.profiles)?;
        for tuple in profiles {
            logger.info(&format!("Preparing profile: {}", tuple.2));
            Self::collect_profile(
                config_map,
                &mut commands_runner,
                &mut all_entries,
                behaviour,
                tuple,
            )
            .context(format!("Profile: {}", tuple.2))?;
        }

        Ok(PlanContext {
            // profile_ctxs,
            commands_runner,
            all_entries,
        })
    }
    pub fn deploy(self) -> Result<()> {
        // let mut backuper = Backuper::init(self.base_path, self.name.to_string(), args.is_dry_run)?;
        self.commands_runner.run_pre_commands()?;

        self.all_entries.copy_and_link()?;

        self.commands_runner.run_post_commands()?;
        Ok(())
    }
    fn collect_profile(
        config_map: &'a ConfigMap<'_>,
        commands_runner: &mut CommandsRunner<'a>,
        all_entries: &mut AllEntries<'a>,
        behaviour: crate::model::Behaviour,
        tuple: (&'a Sketch, usize, &str),
    ) -> Result<(), anyhow::Error> {
        let (profile, idx, profile_name) = tuple;
        let base_path = config_map.get_base_path(idx)?;
        let behaviour = behaviour.override_by(&profile.override_behaviour);
        let stop_at_commands_error = behaviour.stop_at_commands_error.unwrap();
        let envrironment = &profile.environment;
        let commands_path =
            PathResolver::resolve_from_or_base(base_path, &profile.commands_path)?.into_pathbuf();
        commands_runner.add_context(CommandsContext::new(
            envrironment,
            commands_path,
            stop_at_commands_error,
            &profile.pre_build_commands,
            &profile.post_build_commands,
        ));
        all_entries.add_profile(profile, base_path, &behaviour, profile_name)?;
        Ok(())
    }
}
