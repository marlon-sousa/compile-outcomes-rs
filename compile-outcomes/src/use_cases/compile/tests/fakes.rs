//! Test support: a disk and a Cargo whose answers each test gives, recording what the service asks of them.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::domain::{BuildReport, Error, Project};
use crate::ports::{Cargo, Disk};

/// A disk whose files, folders and names are given, and which records each project it writes and each folder it removes.
#[derive(Default)]
pub(super) struct FakeDisk {
    pub(super) unreadable: Vec<PathBuf>,
    pub(super) folder: BTreeSet<PathBuf>,
    pub(super) names: BTreeMap<PathBuf, BTreeSet<OsString>>,
    pub(super) removed: RefCell<Vec<PathBuf>>,
    pub(super) refuses_writing: bool,
    pub(super) written: RefCell<Vec<Project>>,
}

impl Disk for FakeDisk {
    fn readable(&self, file: &Path) -> Result<(), String> {
        if self.unreadable.iter().any(|unreadable| unreadable == file) {
            Err("gone".to_owned())
        } else {
            Ok(())
        }
    }

    fn files_in(&self, _folder: &Path) -> Result<BTreeSet<PathBuf>, String> {
        Ok(self.folder.clone())
    }

    fn names_in(&self, folder: &Path) -> Result<Option<BTreeSet<OsString>>, Error> {
        Ok(self.names.get(folder).cloned())
    }

    fn remove(&self, folder: &Path) -> Result<(), Error> {
        self.removed.borrow_mut().push(folder.to_owned());
        Ok(())
    }

    fn write(&self, project: &Project) -> Result<(), Error> {
        if self.refuses_writing {
            return Err(Error::Unwritable {
                path: project.root.clone(),
                reason: "read-only".to_owned(),
            });
        }
        self.written.borrow_mut().push(project.clone());
        Ok(())
    }
}

/// Cargo that reports the given crates built, or cannot be started, and records each project it builds.
#[derive(Default)]
pub(super) struct FakeCargo {
    pub(super) built: Vec<String>,
    pub(super) missing: bool,
    pub(super) builds: RefCell<Vec<Project>>,
}

impl Cargo for FakeCargo {
    fn build(&self, project: &Project) -> Result<BuildReport, Error> {
        self.builds.borrow_mut().push(project.clone());
        if self.missing {
            return Err(Error::CargoNotStarted {
                program: PathBuf::from("cargo"),
                reason: "No such file or directory".to_owned(),
            });
        }
        Ok(BuildReport {
            built: self.built.clone(),
            messages: Vec::new(),
            error_output: String::new(),
        })
    }
}
