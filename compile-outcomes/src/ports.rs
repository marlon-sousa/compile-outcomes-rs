//! Port: what the service needs the world to do, so that its tests can give it a fake world.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::domain::{BuildReport, Error, Project, ProjectFolder};

/// The disk: the snippet files and folders a compilation names, and the project it writes.
pub(crate) trait Disk {
    /// Nothing when the file can be read, otherwise the reason the system gave, a folder being no file.
    fn readable(&self, file: &Path) -> Result<(), String>;

    /// Every file of `folder`, not looking into subfolders, or the reason the system gave.
    fn files_in(&self, folder: &Path) -> Result<BTreeSet<PathBuf>, String>;

    /// What is at `root` now, or the error naming it when it cannot be read.
    fn project_folder(&self, root: &Path) -> Result<ProjectFolder, Error>;

    /// Writes the project with the owner marker at its root, replacing whatever an earlier compilation left there.
    fn write(&self, project: &Project) -> Result<(), Error>;
}

/// Cargo, which builds a project and reports what the compiler said.
pub(crate) trait Cargo {
    fn build(&self, project: &Project) -> Result<BuildReport, Error>;
}
