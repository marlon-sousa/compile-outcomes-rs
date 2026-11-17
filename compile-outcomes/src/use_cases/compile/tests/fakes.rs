//! Test support: a disk and a Cargo whose answers each test gives, recording what the service asks of them.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::domain::{BuildReport, Error, Project};
use crate::ports::{Cargo, Disk};

/// A disk whose files, folders and names are given, and which records each project it writes.
#[derive(Default)]
pub(super) struct FakeDisk {
    pub(super) unreadable: Vec<PathBuf>,
    pub(super) folder: BTreeSet<PathBuf>,
    pub(super) names: BTreeMap<PathBuf, BTreeSet<OsString>>,
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

    fn names_in(&self, folder: &Path) -> Result<Option<BTreeSet<OsString>>, String> {
        if self
            .unreadable
            .iter()
            .any(|unreadable| unreadable == folder)
        {
            Err("gone".to_owned())
        } else {
            Ok(self.names.get(folder).cloned())
        }
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

/// Cargo that reports the given crates built, or cannot be started, or cannot clean, and records each project it builds and each step it takes.
#[derive(Default)]
pub(super) struct FakeCargo {
    pub(super) built: Vec<String>,
    pub(super) missing: bool,
    pub(super) refuses_cleaning: bool,
    pub(super) builds: RefCell<Vec<Project>>,
    pub(super) steps: RefCell<Vec<Step>>,
}

/// What the service asked Cargo to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Step {
    Clean,
    Build,
}

impl Cargo for FakeCargo {
    fn clean(&self, project: &Project) -> Result<(), Error> {
        self.steps.borrow_mut().push(Step::Clean);
        if self.refuses_cleaning {
            return Err(Error::TargetFolderNotCleaned {
                folder: project.target_dir.clone(),
                reason: "busy".to_owned(),
            });
        }
        Ok(())
    }

    fn build(&self, project: &Project) -> Result<BuildReport, Error> {
        self.steps.borrow_mut().push(Step::Build);
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
