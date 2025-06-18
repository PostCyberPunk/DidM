//TODO:Should i create an error type for this?
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::log::Logger;
pub struct CommandExecutor<'a> {
    pub environment: &'a HashMap<String, String>,
    pub path: &'a Path,
    pub command: &'a String,
}

impl<'a> CommandExecutor<'a> {
    pub fn run(&self) -> io::Result<Output> {
        Command::new("sh")
            .envs(self.environment)
            .current_dir(&self.path)
            .arg("-c")
            .arg(self.command)
            .output()
    }
}
pub struct CommandsContext<'a> {
    pub environment: &'a HashMap<String, String>,
    pub path: PathBuf,
    pub stop_at_commands_error: bool,
    pub pre_commands: &'a Vec<String>,
    pub post_commands: &'a Vec<String>,
}
impl<'a> CommandsContext<'a> {
    pub fn run(&self, cmds: &Vec<String>, logger: &'a Logger, is_dryrun: bool) -> Result<()> {
        let environment = self.environment;
        let path = &self.path;
        logger.debug(&format!("command path {}", path.display()));
        for cmd in cmds {
            logger.info(&format!("Executing: {}", cmd));

            if is_dryrun {
                logger.info(&format!("(dry-run): {}", cmd));
                continue;
            }

            let executor = CommandExecutor {
                environment,
                path,
                command: cmd,
            };

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
                            return Err(anyhow!("Command execution failed:{}", cmd));
                        }
                    } else {
                        logger.info("Command execution success");
                    }
                }
                Err(e) => {
                    logger.error(&format!("Command {} with error: {}", cmd, e));
                    if self.stop_at_commands_error {
                        return Err(anyhow!("Command execution failed:{}", cmd));
                    }
                }
            }
        }
        Ok(())
    }
}
pub struct CommandsRunner<'a> {
    pub context: Vec<CommandsContext<'a>>,
    pub logger: &'a Logger,
    pub is_dryrun: bool,
}
impl<'a> CommandsRunner<'a> {
    pub fn new(logger: &'a Logger, is_dryrun: bool) -> Self {
        Self {
            context: Vec::new(),
            logger,
            is_dryrun,
        }
    }
    pub fn run_pre_commands(&self) -> Result<()> {
        //FIX:!!!!!!
        // if self.pre_commands.is_empty() {
        //     return Ok(());
        // }
        if self.context.is_empty() {
            return Ok(());
        }
        self.logger.info("Running pre commands");
        for ctx in &self.context {
            ctx.run(ctx.pre_commands, self.logger, self.is_dryrun)?;
        }
        Ok(())
    }
    pub fn run_post_commands(&self) -> Result<()> {
        if self.context.is_empty() {
            return Ok(());
        }
        self.logger.info("Running post commands");
        for ctx in &self.context {
            ctx.run(ctx.post_commands, self.logger, self.is_dryrun)?;
        }
        Ok(())
    }
}
