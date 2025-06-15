use std::path::PathBuf;

use super::args::PlanArgs;
use crate::config::ConfigMap;
use crate::helpers::Helpers;
use crate::model::{Behaviour, DidmConfig, Plan, Profile};
use crate::path::PathBufExtension;
use crate::profile::{Backuper, ProfileContext};
use crate::{
    commands::{CommandsContext, CommandsRunner},
    log::Logger,
};
use anyhow::{Context, Result};

pub struct PlanContext<'a> {
    pub plan: &'a Plan,
    pub name: &'a str,
    pub commands_path: PathBuf,
    pub profiles: Vec<(&'a Profile, usize, &'a str)>,
    pub behaviour: Behaviour,
    pub configs: &'a [DidmConfig],
    pub args: &'a PlanArgs,
    pub logger: &'a Logger,
    pub helpers: &'a Helpers,
}

impl<'a> PlanContext<'a> {
    pub fn new(
        plan_name: &'a str,
        config_map: &'a ConfigMap,
        args: &'a PlanArgs,
        logger: &'a Logger,
    ) -> Result<Self> {
        logger.info(&format!("Deploying plan : {} ...", plan_name));
        let main_config = config_map.main_config;
        let plan = config_map.get_plan(plan_name)?;
        let profiles = config_map.get_profiles(&plan.profiles)?;
        let behaviour = Behaviour::merge(&main_config.behaviour, &plan.override_behaviour);

        let base_path = &main_config.base_path;
        let commands_path = base_path.resolve_or_from(&plan.commands_path)?;
        Ok(PlanContext {
            plan,
            name: plan_name,
            profiles,
            commands_path,
            behaviour,
            configs: config_map.configs,
            args,
            logger,
            helpers: &config_map.helpers,
        })
    }
    pub fn deploy(&self) -> Result<()> {
        let logger = self.logger;
        let plan = self.plan;
        let envrironment = &plan.environment;
        let args = self.args;
        let stop_at_commands_error = self.behaviour.stop_at_commands_error.unwrap();
        let mut backuper = Backuper::init(
            self.configs[0].base_path.clone(),
            self.name.to_string(),
            args.is_dry_run,
        )?;
        let cmds_runner = CommandsRunner::new(
            CommandsContext {
                environment: envrironment,
                path: &self.commands_path,
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
            let behaviour = match &profile.override_behaviour {
                Some(b) => &self.behaviour.override_by(b),
                None => &self.behaviour,
            };
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
