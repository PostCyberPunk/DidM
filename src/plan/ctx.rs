use std::path::PathBuf;

use super::args::PlanArgs;
use super::error::PlanError;
use crate::model::{Behaviour, DidmConfig, Plan, Profile, behaviour::Meger};
use crate::{commands::CommandsContext, log::Logger};
use anyhow::Result;

pub struct ProfileContext<'a> {
    pub profile: Profile,
    pub base_path: &'a PathBuf,
    pub behaviour: Behaviour,
    pub args: PlanArgs,
}

pub struct PlanContext<'a> {
    pub plan: &'a Plan,
    pub commands_path: PathBuf,
    pub profiles: Vec<(&'a Profile, usize)>,
    pub behaviour: Behaviour,
    pub args: &'a PlanArgs,
    pub logger: &'a Logger,
}

impl<'a> PlanContext<'a> {
    pub fn new(
        plan_name: &str,
        configs: &'a [DidmConfig],
        args: &'a PlanArgs,
        logger: &'a Logger,
    ) -> Result<Self> {
        let plan = find_plan(plan_name, configs)?;
        let profiles = get_profiles(plan, configs)?;
        let behaviour = Meger(&configs[0].behaviour, &plan.override_behaviour);

        let base_path = &configs[0].base_path;
        let commands_path = match &plan.commands_path {
            //FIX:this only accept relative path
            Some(dir) => base_path.join(dir),
            None => base_path.clone(),
        };
        Ok(PlanContext {
            plan,
            profiles,
            commands_path,
            behaviour,
            args,
            logger,
        })
    }
    pub fn deploy(&self) -> Result<()> {
        let logger = self.logger;
        logger.info("Deploying ...");
        let plan = self.plan;
        let envrironment = &plan.environment;
        let args = self.args;
        let stop_at_commands_error = self.behaviour.stop_at_commands_error.unwrap();
        let cmds_ctx = CommandsContext {
            environment: envrironment,
            path: &self.commands_path,
            logger,
            args,
            stop_at_commands_error,
        };
        logger.info("Executing pre-build-commands ...");
        cmds_ctx.run(&plan.pre_build_commands)?;
        logger.info("Executing post-build-commands ...");
        cmds_ctx.run(&plan.post_build_commands)?;
        Ok(())
    }
}

fn find_plan<'a>(plan_name: &str, configs: &'a [DidmConfig]) -> Result<&'a Plan> {
    configs
        .iter()
        .find_map(|config| config.plans.get(plan_name)) // Gets a reference to Plan
        .ok_or_else(|| PlanError::PlanNotFound.into()) // Handles error
}

fn get_profiles<'a>(
    plan: &'a Plan,
    configs: &'a [DidmConfig],
) -> Result<Vec<(&'a Profile, usize)>> {
    let profile_map: std::collections::HashMap<&str, (&Profile, usize)> = configs
        .iter()
        .enumerate()
        .flat_map(|(idx, config)| {
            config
                .profiles
                .iter()
                .map(move |(name, profile)| (name.as_str(), (profile, idx)))
        })
        .collect();

    plan.profiles
        .iter()
        .map(|profile_name| {
            profile_map
                .get(profile_name.as_str())
                .ok_or_else(|| PlanError::ProfileNotFound(profile_name.to_string()).into())
                .map(|&(profile, config_id)| (profile, config_id))
        })
        .collect()
}
