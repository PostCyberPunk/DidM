use crate::{
    entries::{Entry, apply_strategy::ApplyStrategy},
    utils::PathExtension,
};
use anyhow::Result;
use std::{fs, path::Path};
use std::{io, marker::PhantomData};
use tracing::{error, info, warn};

pub struct EntryList<S: ApplyStrategy> {
    entries: Vec<Entry>,
    _marker: PhantomData<S>,
}

impl<S: ApplyStrategy> EntryList<S> {
    pub fn new() -> Self {
        Self {
            entries: vec![],
            _marker: PhantomData,
        }
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn add_entries(&mut self, entries: Vec<Entry>) {
        self.entries.extend(entries);
    }

    pub fn apply_entries(&self, is_dryrun: bool) -> Result<()> {
        for entry in &self.entries {
            let source = &entry.source_path;
            let target = &entry.target_path;

            //Skip existed target
            if target.exists() && !entry.overwrite_existed {
                info!("Skipped existed: {}", target.display());
                //Retrun result::SkipExsit
                continue;
            }

            //Continue if this is Dry-run
            //NOTE: what about preview?
            if is_dryrun {
                warn!(
                    "Dry-run [{}]: {} -> {}",
                    S::hint(),
                    source.display(),
                    target.display()
                );
                //Return::OK
                continue;
            }

            //remove exsits target fist
            if target.exists() {
                match remove_target(target) {
                    Err(e) => {
                        error!(
                            "Skipped {} \n Failed to remove target, \n reason: {}",
                            target.display(),
                            e
                        );
                        //Return::SkipWithError FailedToRemove
                        continue;
                    }
                    Ok(_) => {
                        //Check again see if file was removed
                        if target.exists() {
                            error!(
                                "Skipped {} \n Failed to remove target, \n reason: Unknown,Please Remove it mannually",
                                target.display(),
                            );
                            //Return::SkipWithError  FailedToRemove
                            continue;
                        }
                    }
                }
            }

            if let Err(e) = target.ensure_parent_exists() {
                error!(
                    "Skippied {:?} \n Failed to create parent folder,\n reason: {}",
                    target, e
                );
                //Return::SkipWithError
                continue;
            }

            if let Err(e) = S::apply(target, source) {
                error!(
                    "Failed [{}]: {} -> {}, error: {}",
                    S::hint(),
                    source.display(),
                    target.display(),
                    e
                );
                //Return::ApplyWithErorr or skip with error?
                continue;
            }

            info!(
                "Applied [{}]: {} -> {}",
                S::hint(),
                source.display(),
                target.display()
            );
            //Retrun::Applied
        }
        //Collect Vec<ApplyResult>
        Ok(())
    }
}
fn remove_target(target: &Path) -> io::Result<()> {
    match target.is_dir() {
        true => fs::remove_dir_all(target),
        //NOTE:Symlink will also be removed here,but what about other type?
        false => fs::remove_file(target),
    }
}
