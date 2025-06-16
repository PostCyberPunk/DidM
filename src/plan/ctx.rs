use super::args::PlanArgs;
use crate::config::ConfigMap;
use crate::helpers::{Helpers, ResolvedPath};
use crate::model::{Behaviour, Plan, Profile};
use crate::profile::{Backuper, ProfileContext};
use crate::{
    commands::{CommandsContext, CommandsRunner},
    log::Logger,
};
use anyhow::{Context, Result};

pub struct PlanContext<'a> {
    pub args: &'a PlanArgs,
    pub logger: &'a Logger,
    pub base_path: &'a ResolvedPath,
    pub plan: &'a Plan,
    pub name: &'a str,
    pub behaviour: Behaviour,
    pub helpers: &'a Helpers,
    pub commands_path: ResolvedPath,
    pub profiles: Vec<(&'a Profile, usize, &'a str)>,
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
        let helpers = config_map.get_helpers();

        let commands_path = helpers
            .path_resolver
            .resolve_from_or_base(base_path, &plan.commands_path)?;

        let profiles = config_map.get_profiles(&plan.profiles)?;

        let behaviour = config_map
            .get_main_behaviour()
            .override_by(&plan.override_behaviour);

        Ok(PlanContext {
            args,
            logger,
            helpers,
            base_path,
            plan,
            name: plan_name,
            profiles,
            commands_path,
            behaviour,
        })
    }
    pub fn deploy(&self) -> Result<()> {
        let logger = self.logger;
        let plan = self.plan;
        let envrironment = &plan.environment;
        let args = self.args;
        let stop_at_commands_error = self.behaviour.stop_at_commands_error.unwrap();
        let mut backuper = Backuper::init(self.base_path, self.name.to_string(), args.is_dry_run)?;
        let cmds_runner = CommandsRunner::new(
            CommandsContext {
                environment: envrironment,
                path: &self.commands_path.get(),
                logger,
                args,
                stop_at_commands_error,
            },
            &plan.pre_build_commands,
            &plan.post_build_commands,
        );
        cmds_runner.run_pre_commands()?;

        // Apply profiles
        //FIX: initialize all profiles to a Vec,then apply
        for (profile, idx, profile_name) in self.profiles.iter() {
            logger.info(&format!("Applying profile: {}", profile_name));
            let behaviour = &self.behaviour.override_by(&profile.override_behaviour);
            let mut profile_ctx =
                ProfileContext::new(profile_name, *idx, profile, self, behaviour, &mut backuper);
            profile_ctx
                .apply()
                .context(format!("Profile apply failed:{}", profile_name))?;
        }

        cmds_runner.run_post_commands()?;
        Ok(())
    }
}
