use anyhow::{Context, Result};
use chrono::Local;
use std::{
    fs::{self},
    path::{Path, PathBuf},
};
use thiserror::Error;

use crate::helpers::PathExtension;
use crate::{helpers::ResolvedPath, log::Logger};

//FIX: the ctx should be borrow from plan, not from profile
//initialize in profile then it can be imutable
pub struct Backuper {
    ctx: Option<BackuperContext>,
    base_dir: PathBuf,
    is_dryrun: bool,
}
pub struct BackuperContext {
    normal_path: PathBuf,
    extra_path: PathBuf,
    empty_path: PathBuf,
    null_path: PathBuf,
}

impl Backuper {
    pub fn init(base_path: &ResolvedPath, plan_name: String, is_dryrun: bool) -> Result<Self> {
        base_path
            .get()
            .check_dir()
            .and_then(|_| base_path.get().check_permission())
            .with_context(|| BackupError::InitializeFailed)?;

        let now = Local::now().format("%Y_%m_%d_%H_%M_%S").to_string();
        let base_dir = base_path
            .get()
            .join(".didm_backup")
            .join(format!("plan_{}-{}", plan_name, now));

        Ok(Self {
            ctx: None,
            base_dir,
            is_dryrun,
        })
    }
    pub fn set_ctx(&mut self, prefix: String) {
        let base_dir = &self.base_dir.join(prefix);
        let normal_path = base_dir.join("normal");
        let extra_path = base_dir.join("extra");
        let empty_path = base_dir.join("empty");
        let null_path = base_dir.join("null");
        self.ctx = Some(BackuperContext {
            normal_path,
            extra_path,
            empty_path,
            null_path,
        });
    }
    pub fn drop_ctx(&mut self, logger: &Logger) {
        if self.base_dir.exists() {
            logger.warn(&format!("Backup created at :{}", self.base_dir.display()));
        }
        self.ctx = None;
    }
    // fn get_ctx(&self) -> Result<&BackuperContext> {
    //     match &self.ctx {
    //         Some(ctx) => Ok(ctx),
    //         None => Err(BackupError::BackupContextIsNotSet.into()),
    //     }
    // }
    fn do_backup(&self, src: &Path, dest: &Path, logger: &Logger) -> Result<()> {
        if dest.exists() {
            return Err(BackupError::BackupExsisted(dest.display().to_string()).into());
        }
        if !self.is_dryrun {
            dest.ensure_parent_exists()?;
            fs::rename(src, dest)?;
            //FIX: if src.exists() still there
        }
        logger.warn(&format!(
            "Backup {} to\n        {}",
            src.display(),
            dest.display()
        ));
        Ok(())
    }
    pub fn backup<F>(&self, src: &Path, relative: &Path, logger: &Logger, pred: F) -> Result<()>
    where
        F: Fn() -> bool,
    {
        let ctx = match &self.ctx {
            Some(ctx) => ctx,
            None => return Ok(()),
        };
        if !pred() {
            return Ok(());
        }
        if src.is_symlink() {
            logger.warn(&format!(
                "Symlink will be removed at:{}\n        Target:{}",
                src.display(),
                src.read_link().map_or_else(
                    |_| String::from("Invalid symlink target"),
                    |p| p.display().to_string()
                )
            ));
            return Ok(());
        }
        let backup_path = ctx.normal_path.join(relative);

        self.do_backup(src, &backup_path, logger)
            .with_context(|| BackupError::Failed(src.display().to_string()))?;
        Ok(())
    }
}
#[derive(Error, Debug)]
pub enum BackupError {
    #[error("Failed to initialize backuper")]
    InitializeFailed,
    #[error("An backup already exists: {0}")]
    BackupExsisted(String),
    #[error("Failed to backup :{0}")]
    Failed(String),
    //
    // #[error("Failed to strip prefix from path: {0}")]
    // StripPrefixError(String),
}
