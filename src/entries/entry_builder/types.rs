use async_trait::async_trait;

use crate::utils::ResolvedPath;

use crate::bakcup::BackupManager;

pub struct EntryBuilderCtx<'a> {
    pub backup_manager: Option<&'a BackupManager>,
    pub source_root: ResolvedPath,
    pub target_root: ResolvedPath,
    pub overwrite: bool,
}
#[async_trait]
pub trait BuildStrategy: Sized {
    // fn builder<'a>(
    //     ctx: &'a EntryBuilderCtx<'a>,
    //     source: PathBuf,
    //     target: PathBuf,
    // ) -> Result<EntryBuilder<'a, Self>>;

    // async fn do_backup(builder: &EntryBuilder<'_, Self>) -> BackupState;
}
