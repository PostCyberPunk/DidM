use std::path::PathBuf;

use super::args::PlanArgs;
use super::error::PlanError;
use crate::config::ConfigMap;
use crate::model::{Behaviour, DidmConfig, Plan, Profile, behaviour};
use crate::path::PathBufExtension;
use crate::{
    commands::{CommandsContext, CommandsRunner},
    log::Logger,
};
use anyhow::Result;

pub struct PlanContext<'a> {
    pub plan: &'a Plan,
    pub commands_path: PathBuf,
    pub profiles: Vec<(&'a Profile, usize, &'a str)>,
    pub behaviour: Behaviour,
    pub args: &'a PlanArgs,
    pub logger: &'a Logger,
}

impl<'a> PlanContext<'a> {
    pub fn new(
        plan_name: &str,
        config_map: &'a ConfigMap,
        args: &'a PlanArgs,
        logger: &'a Logger,
    ) -> Result<Self> {
        logger.info(&format!("Deploying plan : {} ...", plan_name));
        let main_config = config_map.main_config;
        let plan = config_map.get_plan(plan_name)?;
        let profiles = config_map.get_profiles(&plan.profiles)?;
        let behaviour = behaviour::Meger(&main_config.behaviour, &plan.override_behaviour);

        let base_path = &main_config.base_path;
        let commands_path = match &plan.commands_path {
            //FIX:this only accept relative path
            Some(dir) => base_path.join(dir).resolve()?,
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
        let plan = self.plan;
        let envrironment = &plan.environment;
        let args = self.args;
        let stop_at_commands_error = self.behaviour.stop_at_commands_error.unwrap();
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

        cmds_runner.run_post_commands()?;
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
