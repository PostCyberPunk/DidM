use anyhow::{Context, Result};
use chrono::Local;
use std::{
    fs::{self},
    path::{Path, PathBuf},
};
use thiserror::Error;

use crate::utils::ResolvedPath;
use crate::{log::Logger, utils::PathExtension};

use super::SouceType;

//FIX: the ctx should be borrow from composition, not from sketch
//initialize in sketch then it can be imutable
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
// TODO: this is not apply to entry
pub enum BackupState {
    Ok,
    Skip,
    Symlink,
    Backuped,
}
pub struct BackupRoot {
    base_dir: PathBuf,
    is_dryrun: bool,
}
impl BackupRoot {
    pub fn new(base_path: &ResolvedPath, comp_name: String, is_dryrun: bool) -> Result<Self> {
        //Make sure we can write at the base path
        base_path
            .get()
            .check_dir()
            .and_then(|_| base_path.get().check_permission())
            .with_context(|| BackupError::InitializeFailed)?;
        //TODO: we can get data by meta data,we can have a better name
        let now = Local::now().format("%Y_%m_%d_%H_%M_%S").to_string();
        let base_dir = base_path
            .get()
            .join(".didm_backup")
            .join(format!("composition_{}-{}", comp_name, now));
        Ok(Self {
            base_dir,
            is_dryrun,
        })
    }
    pub fn has_bakcup(self, logger: &Logger) {
        if self.base_dir.exists() {
            logger.warn(&format!("Backup created at :{}", self.base_dir.display()));
        }
    }
}

pub struct Backuper {
    is_dryrun: bool,
    normal_path: PathBuf,
    empty_path: PathBuf,
    null_path: PathBuf,
    extra_path: PathBuf,
}

impl Backuper {
    pub fn init(root_info: &BackupRoot, dir_name: String) -> Result<Self> {
        let base_dir = &root_info.base_dir.join(dir_name);
        let is_dryrun = root_info.is_dryrun;
        let normal_path = base_dir.join("normal");
        let extra_path = base_dir.join("extra");
        let empty_path = base_dir.join("empty");
        let null_path = base_dir.join("null");
        Ok(Self {
            is_dryrun,
            normal_path,
            extra_path,
            empty_path,
            null_path,
        })
    }

    pub fn backup_normal(
        &self,
        src: &Path,
        relative: &Path,
        logger: &Logger,
    ) -> Result<BackupState> {
        if Self::check_symlink(src, logger)? {
            return Ok(BackupState::Symlink);
        }

        let dest_path = self.normal_path.join(relative);

        self.do_backup(src, &dest_path, logger)?;
        Ok(BackupState::Backuped)
    }

    pub fn backup_other(
        &self,
        src: &Path,
        logger: &Logger,
        src_type: SouceType,
    ) -> Result<BackupState> {
        if Self::check_symlink(src, logger)? {
            return Ok(BackupState::Symlink);
        }
        let _dir = match src_type {
            SouceType::Normal => {
                return Err(BackupError::BugWrongType.into());
            }
            SouceType::Null => &self.null_path,
            SouceType::Empty => &self.empty_path,
            SouceType::Extra => &self.extra_path,
        };
        let encoded_path = urlencoding::encode(&src.to_string_lossy()).into_owned();
        let backup_path = _dir.join(encoded_path);

        self.do_backup(src, &backup_path, logger)?;
        Ok(BackupState::Backuped)
    }

    fn do_backup(&self, src: &Path, dest: &Path, logger: &Logger) -> Result<()> {
        if dest.exists() {
            return Err(BackupError::BackupExsisted(dest.display().to_string()).into());
        }
        if !self.is_dryrun {
            dest.ensure_parent_exists()
                .context("Failed to create parent directory")?;
            fs::rename(src, dest).context("Failed to move target")?;
        }

        logger.warn(&format!(
            "Backup {} to\n        {}",
            src.display(),
            dest.display()
        ));
        Ok(())
    }

    fn check_symlink(src: &Path, logger: &Logger) -> Result<bool> {
        if src.is_symlink() {
            logger.warn(&format!(
                "Symlink will be removed at:{}\n        Target:{}",
                src.display(),
                src.read_link().map_or_else(
                    |_| String::from("Invalid symlink target"),
                    |p| p.display().to_string()
                )
            ));
            return Ok(true);
        }
        Ok(false)
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
    //TODO:! this could be avoid with right abstraction model
    #[error("BUG:Calling normal entry on backup_other")]
    BugWrongType,
}
