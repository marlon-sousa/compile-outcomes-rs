//! Service: runs a compilation through the ports and the policies.

use std::path::Path;

use crate::domain::{
    Build, Compilation, Error, Errors, Found, Outcomes, Project, Snippet, SnippetSource,
};
use crate::ports::{Cargo, Disk};
use crate::use_cases::{naming, outcomes, ownership, project};

/// The outcome of every snippet of `compilation`, or every problem with its description, or the first problem while compiling it.
pub(crate) fn compile(
    compilation: &Compilation,
    disk: &impl Disk,
    cargo: &impl Cargo,
) -> Result<Outcomes, Errors> {
    let found = compilation
        .sources()
        .iter()
        .map(|source| find(source, disk))
        .collect();
    let lockfile = compilation
        .lockfile_path()
        .as_deref()
        .and_then(|lockfile| unreadable_lockfile(lockfile, disk));
    let snippets = described(naming::snippets(found), lockfile)?;
    let project = project::project(compilation, &snippets);
    untouched_unless_owned(&project, compilation.build(), disk)?;
    disk.write(&project)?;
    if compilation.build().is_clean() {
        cargo.clean(&project)?;
    }
    let report = cargo.build(&project)?;
    Ok(outcomes::outcomes(&snippets, report)?)
}

/// Refuses a project folder compile-outcomes did not write, and for a clean build a target folder Cargo did not make, before either is changed.
fn untouched_unless_owned(project: &Project, build: Build, disk: &impl Disk) -> Result<(), Error> {
    let held = match disk.names_in(&project.root) {
        Ok(held) => held,
        Err(reason) => {
            return Err(Error::Unwritable {
                path: project.root.clone(),
                reason,
            });
        }
    };
    if ownership::project_folder(held.as_ref()).is_foreign() {
        return Err(Error::ForeignProjectFolder {
            folder: project.root.clone(),
        });
    }
    if build.is_clean() {
        let held = match disk.names_in(&project.target_dir) {
            Ok(held) => held,
            Err(reason) => {
                return Err(Error::TargetFolderNotCleaned {
                    folder: project.target_dir.clone(),
                    reason,
                });
            }
        };
        if ownership::target_folder(held.as_ref()).is_foreign() {
            return Err(Error::ForeignTargetFolder {
                folder: project.target_dir.clone(),
            });
        }
    }
    Ok(())
}

fn find(source: &SnippetSource, disk: &impl Disk) -> Found {
    match source {
        SnippetSource::File(file) => match disk.readable(file) {
            Ok(()) => Found::File(file.clone()),
            Err(reason) => Found::UnreadableSnippet {
                file: file.clone(),
                reason,
            },
        },
        SnippetSource::Folder(folder) => match disk.files_in(folder) {
            Ok(files) => Found::Folder(files),
            Err(reason) => Found::UnreadableFolder {
                folder: folder.clone(),
                reason,
            },
        },
    }
}

fn unreadable_lockfile(lockfile: &Path, disk: &impl Disk) -> Option<Error> {
    let reason = disk.readable(lockfile).err()?;
    Some(Error::UnreadableLockfile {
        lockfile: lockfile.to_owned(),
        reason,
    })
}

/// The snippets, or every problem with the description, the lockfile's last.
fn described(
    named: Result<Vec<Snippet>, Errors>,
    lockfile: Option<Error>,
) -> Result<Vec<Snippet>, Errors> {
    match (named, lockfile) {
        (named, None) => named,
        (Ok(_), Some(problem)) => Err(problem.into()),
        (Err(errors), Some(problem)) => Err(errors.and(problem)),
    }
}

#[cfg(test)]
mod tests;
