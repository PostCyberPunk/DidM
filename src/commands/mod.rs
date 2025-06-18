//TODO:Should i create an error type for this?
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::log::Logger;
struct CommandExecutor<'a> {
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
    environment: &'a HashMap<String, String>,
    path: PathBuf,
    stop_at_commands_error: bool,
    pre_commands: &'a Vec<String>,
    post_commands: &'a Vec<String>,
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
    context: Vec<CommandsContext<'a>>,
    logger: &'a Logger,
    is_dryrun: bool,
}
impl<'a> CommandsRunner<'a> {
    pub fn new(logger: &'a Logger, is_dryrun: bool) -> Self {
        Self {
            context: Vec::new(),
            logger,
            is_dryrun,
        }
    }

    pub fn add_context(&mut self, context: CommandsContext<'a>) {
        self.context.push(context);
    }

    pub fn run_pre_commands(&self) -> Result<()> {
        self.run_commands("pre", |ctx| ctx.pre_commands)
    }

    pub fn run_post_commands(&self) -> Result<()> {
        self.run_commands("post", |ctx| ctx.post_commands)
    }

    fn run_commands<F>(&self, label: &str, commands_selector: F) -> Result<()>
    where
        F: Fn(&CommandsContext<'a>) -> &'a [String],
    {
        if self.context.is_empty() {
            return Ok(());
        }

        self.logger.info(&format!("Running {} commands", label));

        for ctx in &self.context {
            ctx.run(commands_selector(ctx), self.logger, self.is_dryrun)?;
        }

        Ok(())
    }
}
