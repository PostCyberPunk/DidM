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
    // pub args: &'a PlanArgs,
    // pub logger: &'a Logger,
    // pub config_map: &'a ConfigMap<'a>,
    // pub base_path: &'a ResolvedPath,
    // pub plan: &'a Plan,
    // pub name: &'a str,
    // pub behaviour: Behaviour,
    pub commands_runner: CommandsRunner<'a>,
    pub profile_ctxs: Vec<(&'a str, ProfileContext<'a>)>,
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

        //Get Bhaviour
        let behaviour = config_map
            .get_main_behaviour()
            .override_by(&plan.override_behaviour);

        //Prepare Command runner
        let envrironment = &plan.environment;
        let stop_at_commands_error = behaviour.stop_at_commands_error.unwrap();
        let commands_path = helpers
            .path_resolver
            .resolve_from_or_base(base_path, &plan.commands_path)?
            .into_pathbuf();
        let commands_runner = CommandsRunner::new(
            CommandsContext {
                environment: envrironment,
                path: commands_path,
                logger,
                args,
                stop_at_commands_error,
            },
            &plan.pre_build_commands,
            &plan.post_build_commands,
        );

        //apply profiles
        let profiles = config_map.get_profiles(&plan.profiles)?;
        let mut profile_ctxs = Vec::new();

        for (profile, idx, profile_name) in profiles {
            logger.info(&format!("Applying profile: {}", profile_name));
            let base_path = config_map.get_base_path(idx)?;
            let profile_ctx = ProfileContext::new(
                args,
                logger,
                helpers,
                profile_name,
                base_path,
                profile,
                behaviour.clone(),
            )?;
            profile_ctxs.push((profile_name, profile_ctx));
        }

        Ok(PlanContext {
            // args,
            // logger,
            // helpers,
            // config_map,
            // base_path,
            // plan,
            // name: plan_name,
            // behaviour,
            profile_ctxs,
            commands_runner,
        })
    }
    pub fn deploy(self) -> Result<()> {
        // let mut backuper = Backuper::init(self.base_path, self.name.to_string(), args.is_dry_run)?;
        self.commands_runner.run_pre_commands()?;

        for (profile_name, p) in self.profile_ctxs {
            p.apply()
                .context(format!("Profile apply failed:{}", profile_name))?;
        }
        // self.profile_ctxs.iter().try_for_each(|(_, p)| p.apply())?;

        self.commands_runner.run_post_commands()?;
        Ok(())
    }
}
