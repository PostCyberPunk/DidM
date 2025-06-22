use anyhow::{Context, Result};
use std::{
    fs::{self},
    path::{Path, PathBuf},
};
use tracing::warn;

use crate::{entries::SouceType, utils::PathExtension};

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
    pub async fn bakcup_async(
        &self,
        src: &Path,
        relative: Option<PathBuf>,
        src_type: SouceType,
    ) -> Result<BackupState> {
        if !src.exists() {
            return Ok(BackupState::Ok);
        }
        if Self::check_symlink(src) {
            return Ok(BackupState::Symlink);
        }
        let dest_path = match src_type {
            SouceType::Normal => self.normal_path.join(relative.context("No relative path")?),
            SouceType::Extra => self.get_extra_path(src)?,
            _ => {
                return Ok(BackupState::Skip);
            }
        };

        self.do_backup(src, &dest_path)?;

        Ok(BackupState::Backuped)
    }

    fn get_extra_path(&self, src: &Path) -> Result<PathBuf> {
        let _dir = &self.extra_path;

        let parent_path = src
            .parent()
            .with_context(|| format!("Failed to get parent directory:{:?}", src))?;

        let filename = src
            .file_name()
            .with_context(|| format!("Failed to get file_name:{:?}", src))?;

        let encoded_path = urlencoding::encode(&parent_path.to_string_lossy()).into_owned();
        let dest_path = _dir.join(encoded_path).join(filename);
        Ok(dest_path)
    }

    fn do_backup(&self, src: &Path, dest: &Path) -> Result<()> {
        if dest.exists() {
            return Err(BackupError::BackupExsisted(dest.display().to_string()).into());
        }
        if !self.is_dryrun {
            dest.ensure_parent_exists()
                .context("Failed to create parent directory")?;
            fs::rename(src, dest).context("Failed to move target")?;
        }

        warn!("Backup {} to\n        {}", src.display(), dest.display());
        Ok(())
    }

    fn check_symlink(src: &Path) -> bool {
        if src.is_symlink() {
            warn!(
                "Symlink will be removed at:{}\n        Target:{}",
                src.display(),
                src.read_link().map_or_else(
                    |_| String::from("Invalid symlink target"),
                    |p| p.display().to_string()
                )
            );
            return true;
        }
        false
    }
}
