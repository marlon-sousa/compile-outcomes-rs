//! Adapter: the disk, through `std::fs`, writing the project as `toml` manifests.

mod manifest;

use std::collections::BTreeSet;
use std::env;
use std::ffi::OsString;
use std::fs::{self, DirEntry, ReadDir};
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

use crate::domain::{Error, OWNER_MARKER, Project};
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

    fn names_in(&self, folder: &Path) -> Result<Option<BTreeSet<OsString>>, String> {
        match fs::read_dir(folder) {
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
            listed => listed
                .and_then(names_holding_files)
                .map(Some)
                .map_err(reason),
        }
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

/// The names of the entries that are files or hold one at any depth.
fn names_holding_files(entries: ReadDir) -> io::Result<BTreeSet<OsString>> {
    entries.filter_map(name_if_holding_a_file).collect()
}

fn name_if_holding_a_file(entry: io::Result<DirEntry>) -> Option<io::Result<OsString>> {
    entry.and_then(kept_name).transpose()
}

/// The entry's name, if it holds a file.
fn kept_name(entry: DirEntry) -> io::Result<Option<OsString>> {
    Ok(holds_a_file(&entry)?.then(|| entry.file_name()))
}

/// Whether the entry is a file, or a folder holding one at any depth, stopping at the first file found; anything that is not a folder counts as a file.
fn holds_a_file(entry: &DirEntry) -> io::Result<bool> {
    if !entry.file_type()?.is_dir() {
        return Ok(true);
    }
    fs::read_dir(entry.path())?
        .map(entry_holds_a_file)
        .find(found_or_failed)
        .unwrap_or(Ok(false))
}

fn entry_holds_a_file(entry: io::Result<DirEntry>) -> io::Result<bool> {
    holds_a_file(&entry?)
}

fn found_or_failed(holds: &io::Result<bool>) -> bool {
    !matches!(holds, Ok(false))
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
