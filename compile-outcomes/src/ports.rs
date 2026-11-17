//! Port: what the service needs the world to do, so that its tests can give it a fake world.

use std::collections::BTreeSet;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::domain::{BuildReport, Error, Project};

/// The disk: the snippet files and folders a compilation names, and the project it writes.
pub(crate) trait Disk {
    /// Nothing when the file can be read, otherwise the reason the system gave, a folder being no file.
    fn readable(&self, file: &Path) -> Result<(), String>;

    /// Every file of `folder`, not looking into subfolders, or the reason the system gave.
    fn files_in(&self, folder: &Path) -> Result<BTreeSet<PathBuf>, String>;

    /// The name of every entry of `folder` that is a file or holds one at any depth, `None` when there is no such folder, or the reason the system gave.
    ///
    /// A folder holding only empty folders has no such entry, as one a CI cache stripped of its files, keeping its folders.
    fn names_in(&self, folder: &Path) -> Result<Option<BTreeSet<OsString>>, String>;

    /// Writes the project with the owner marker at its root, replacing whatever an earlier compilation left there.
    fn write(&self, project: &Project) -> Result<(), Error>;
}

/// Cargo, which cleans a project's build output, and builds the project and reports what the compiler said.
pub(crate) trait Cargo {
    fn clean(&self, project: &Project) -> Result<(), Error>;

    fn build(&self, project: &Project) -> Result<BuildReport, Error>;
}
