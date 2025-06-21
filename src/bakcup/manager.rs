use anyhow::{Context, Result};
use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use crate::{entries::SouceType, log::Logger, utils::PathExtension};

use super::{BackupRoot, BackupState, error::BackupError};

//TODO: the ideal way to backup should be
//collect and borrow entries that need to backuped,then deal them together
//to achieve that,we need to add sourcetype to entry
//this can also fix encoding problem by replace encoding with index
//TODO: we also need a summary ,so we can restore backup

//FIX: bad structure 5*24 + 1
pub struct BackupManager {
    normal_path: PathBuf,
    extra_path: PathBuf,
    // empty_path: PathBuf,
    // null_path: PathBuf,
    is_dryrun: bool,
}

//TODO: not an ideal name, but i dont like backuper
impl BackupManager {
    pub fn init(root_info: &BackupRoot, dir_name: String) -> Result<Self> {
        let base_dir = &root_info.base_dir.join(dir_name);
        let is_dryrun = root_info.is_dryrun;
        let normal_path = base_dir.join("normal");
        let extra_path = base_dir.join("extra");
        // let empty_path = base_dir.join("empty");
        // let null_path = base_dir.join("null");
        Ok(Self {
            is_dryrun,
            normal_path,
            extra_path,
            // empty_path,
            // null_path,
        })
    }

    pub fn backup_normal(
        &self,
        src: &Path,
        relative: &Path,
        logger: &Logger,
    ) -> Result<BackupState> {
        if !src.exists() {
            return Ok(BackupState::Ok);
        }
        if Self::check_symlink(src, logger) {
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
        if !src.exists() {
            return Ok(BackupState::Ok);
        }
        if Self::check_symlink(src, logger) {
            return Ok(BackupState::Symlink);
        }
        let _dir = match src_type {
            // SouceType::Normal => {
            //     return Err(BackupError::BugWrongType.into());
            // }
            // SouceType::Null => &self.null_path,
            // SouceType::Empty => &self.empty_path,
            SouceType::Extra => &self.extra_path,
            _ => {
                return Err(BackupError::BugWrongType.into());
            }
        };
        let parent_path = src.parent().unwrap();
        let filename = src.file_name().unwrap();

        let encoded_path = urlencoding::encode(&parent_path.to_string_lossy()).into_owned();
        let backup_path = _dir.join(encoded_path).join(filename);

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

    fn check_symlink(src: &Path, logger: &Logger) -> bool {
        if src.is_symlink() {
            logger.warn(&format!(
                "Symlink will be removed at:{}\n        Target:{}",
                src.display(),
                src.read_link().map_or_else(
                    |_| String::from("Invalid symlink target"),
                    |p| p.display().to_string()
                )
            ));
            return true;
        }
        false
    }
}
