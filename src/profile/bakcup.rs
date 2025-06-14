use anyhow::Result;
use chrono::Local;
use std::{
    fs::{self, metadata},
    path::{Path, PathBuf},
};
use thiserror::Error;

use crate::log::Logger;
use crate::path::PathBufExtension;

pub struct Backuper {
    ctx: Option<BackuperContext>,
    base_dir: PathBuf,
    is_dryrun: bool,
}
pub struct BackuperContext {
    normal_path: PathBuf,
    additional_path: PathBuf,
    empty_path: PathBuf,
    null_path: PathBuf,
}

impl Backuper {
    pub fn init(path: PathBuf, plan_name: String, is_dryrun: bool) -> Result<Self> {
        //REFT: this to to pathbuf ext
        match metadata(&path) {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    return Err(BackupError::PathIsNotDir(path.display().to_string()).into());
                }
            }
            Err(err) => {
                return Err(BackupError::CreateBackupDir(
                    path.display().to_string(),
                    err.to_string(),
                )
                .into());
            }
        }
        let now = Local::now().format("%Y_%m_%d_%H_%M_%S").to_string();
        let base_dir = path
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
        let additional_path = base_dir.join("additional");
        let empty_path = base_dir.join("empty");
        let null_path = base_dir.join("null");
        self.ctx = Some(BackuperContext {
            normal_path,
            additional_path,
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
    fn do_backup(&self, src: &Path, dest: &PathBuf, logger: &Logger) -> Result<()> {
        if dest.exists() {
            return Err(BackupError::BackupExsisted(dest.display().to_string()).into());
        }
        if !self.is_dryrun {
            //REFT: impl this trait for path
            dest.ensure_parent_exists()?;
            fs::rename(src, dest)?;
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
        let backup_path = ctx.normal_path.join(relative);

        self.do_backup(src, &backup_path, logger)?;
        Ok(())
    }
}
#[derive(Error, Debug)]
pub enum BackupError {
    #[error("Failed to create backup directory: {0}\n,Error:{1}")]
    CreateBackupDir(String, String),

    #[error("Failed to create backup directory: {0}")]
    PathIsNotDir(String),
    #[error("An backup already exists: {0}")]
    BackupExsisted(String),
    // #[error("Backuper context is not set")]
    // BackupContextIsNotSet,
    //
    // #[error("Failed to strip prefix from path: {0}")]
    // StripPrefixError(String),
}
