use super::executor::CommandExecutor;
use crate::log::Logger;
use anyhow::Result;
use std::{collections::HashMap, path::PathBuf};

pub struct CommandsContext<'a> {
    environment: &'a HashMap<String, String>,
    path: PathBuf,
    stop_at_commands_error: bool,
    pub pre_commands: &'a Vec<String>,
    pub post_commands: &'a Vec<String>,
}

impl<'a> CommandsContext<'a> {
    pub fn new(
        environment: &'a HashMap<String, String>,
        path: PathBuf,
        stop_at_commands_error: bool,
        pre_commands: &'a Vec<String>,
        post_commands: &'a Vec<String>,
    ) -> Self {
        Self {
            environment,
            path,
            stop_at_commands_error,
            pre_commands,
            post_commands,
        }
    }
    pub fn run(&self, cmds: &[String], logger: &'a Logger, is_dryrun: bool) -> Result<()> {
        if cmds.is_empty() {
            return Ok(());
        }
        let environment = self.environment;
        let path = &self.path;
        logger.debug(&format!("command path {}", path.display()));
        for cmd in cmds {
            logger.info(&format!("Executing: {}", cmd));

            if is_dryrun {
                logger.info(&format!("(dry-run): {}", cmd));
                continue;
            }

            let executor = CommandExecutor::new(environment, path, cmd);

            let output_result = executor.run();
            match output_result {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    if !stdout.is_empty() {
                        logger.info(&format!("stdout:\n{}", stdout));
                    }
                    if !stderr.is_empty() {
                        logger.warn(&format!("stderr:\n{}", stderr));
                    }
                    if !output.status.success() {
                        logger.error(&format!(
                            "Command execution failed{} exit code:{}",
                            cmd,
                            output.status.code().unwrap_or(-1)
                        ));
                        if self.stop_at_commands_error {
                            return Err(anyhow::anyhow!("Command execution failed:{}", cmd));
                        }
                    } else {
                        logger.info("Command execution success");
                    }
                }
                Err(e) => {
                    logger.error(&format!("Command {} with error: {}", cmd, e));
                    if self.stop_at_commands_error {
                        return Err(anyhow::anyhow!("Command execution failed:{}", cmd));
                    }
                }
            }
        }
        Ok(())
    }
}
