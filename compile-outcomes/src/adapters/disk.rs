//! Adapter: the disk, through `std::fs`, writing the project as `toml` manifests.

mod manifest;

use std::collections::BTreeSet;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::{self, DirEntry};
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

use crate::domain::{Error, OWNER_MARKER, Project, ProjectFolder};
use crate::ports::Disk;

/// The disk of the machine compile-outcomes runs on.
pub(crate) struct FileSystem;

impl Disk for FileSystem {
    fn readable(&self, file: &Path) -> Result<(), String> {
        if fs::metadata(file).map_err(reason)?.is_dir() {
            Err(reason(io::Error::from(ErrorKind::IsADirectory)))
        } else {
            fs::File::open(file).map(drop).map_err(reason)
        }
    }

    fn files_in(&self, folder: &Path) -> Result<BTreeSet<PathBuf>, String> {
        let mut files: BTreeSet<PathBuf> = fs::read_dir(folder)
            .map_err(reason)?
            .map(path_of)
            .collect::<io::Result<_>>()
            .map_err(reason)?;
        files.retain(|file| file.is_file());
        Ok(files)
    }

    fn project_folder(&self, root: &Path) -> Result<ProjectFolder, Error> {
        let names: BTreeSet<OsString> = match fs::read_dir(root) {
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(ProjectFolder::Missing),
            listed => listed
                .and_then(|entries| entries.map(name_of).collect())
                .map_err(|error| unwritable(root, &error))?,
        };
        Ok(if names.is_empty() {
            ProjectFolder::Empty
        } else if names.contains(OsStr::new(OWNER_MARKER)) {
            ProjectFolder::Owned
        } else {
            ProjectFolder::Foreign
        })
    }

    fn write(&self, project: &Project) -> Result<(), Error> {
        let base = env::current_dir().map_err(|error| unwritable(&project.root, &error))?;
        replace_folder(&project.root)?;
        write_file(&project.root.join(OWNER_MARKER), "")?;
        write_file(
            &project.root.join("Cargo.toml"),
            &manifest::workspace(project),
        )?;
        for package in &project.packages {
            let folder = project.root.join(&package.name);
            create_folder(&folder)?;
            let contents = manifest::package(project, package, &base);
            write_file(&folder.join("Cargo.toml"), &contents)?;
        }
        if let Some(lockfile) = &project.lockfile {
            let copy = project.root.join("Cargo.lock");
            fs::copy(lockfile, &copy).map_err(|error| unwritable(&copy, &error))?;
        }
        Ok(())
    }
}

/// Removes `folder` and what it holds, if it is there, and makes it again, empty.
fn replace_folder(folder: &Path) -> Result<(), Error> {
    match fs::remove_dir_all(folder) {
        Err(error) if error.kind() != ErrorKind::NotFound => Err(unwritable(folder, &error)),
        _ => create_folder(folder),
    }
}

fn create_folder(folder: &Path) -> Result<(), Error> {
    fs::create_dir_all(folder).map_err(|error| unwritable(folder, &error))
}

fn write_file(file: &Path, contents: &str) -> Result<(), Error> {
    fs::write(file, contents).map_err(|error| unwritable(file, &error))
}

fn path_of(entry: io::Result<DirEntry>) -> io::Result<PathBuf> {
    Ok(entry?.path())
}

fn name_of(entry: io::Result<DirEntry>) -> io::Result<OsString> {
    Ok(entry?.file_name())
}

fn reason(error: io::Error) -> String {
    error.to_string()
}

fn unwritable(path: &Path, error: &io::Error) -> Error {
    Error::Unwritable {
        path: path.to_owned(),
        reason: error.to_string(),
    }
}

#[cfg(test)]
mod tests;
