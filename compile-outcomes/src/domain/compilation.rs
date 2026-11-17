//! Entity: the description of a compilation, built step by step.

use std::path::PathBuf;

use getset::{CopyGetters, Getters};

use crate::domain::{Dependency, Edition};

/// The description of one compilation: its snippets, the dependencies they share and the edition they are compiled with.
///
/// Describing a compilation touches nothing on disk; only compiling it does.
#[derive(Debug, Clone, PartialEq, Eq, Getters, CopyGetters)]
pub struct Compilation {
    #[getset(get_copy = "pub(crate)")]
    edition: Edition,
    #[getset(get = "pub(crate)")]
    work_dir: PathBuf,
    #[getset(get = "pub(crate)")]
    sources: Vec<SnippetSource>,
    #[getset(get = "pub(crate)")]
    dependencies: Vec<Dependency>,
    #[getset(get = "pub(crate)")]
    lockfile_path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SnippetSource {
    File(PathBuf),
    Folder(PathBuf),
}

impl Compilation {
    /// Starts a compilation with `edition`, writing its project and its build output in `work_dir`.
    ///
    /// compile-outcomes owns `work_dir`: it replaces the project there at each compilation, and reuses the build output there from one compilation to the next.
    pub fn new(edition: Edition, work_dir: impl Into<PathBuf>) -> Self {
        Self {
            edition,
            work_dir: work_dir.into(),
            sources: Vec::new(),
            dependencies: Vec::new(),
            lockfile_path: None,
        }
    }

    /// Adds the snippet in the file `path`, whatever its extension.
    #[must_use]
    pub fn snippet(mut self, path: impl Into<PathBuf>) -> Self {
        self.sources.push(SnippetSource::File(path.into()));
        self
    }

    /// Adds every file of the folder `dir` whose extension is `rs`, not looking into subfolders.
    #[must_use]
    pub fn snippets_in(mut self, dir: impl Into<PathBuf>) -> Self {
        self.sources.push(SnippetSource::Folder(dir.into()));
        self
    }

    /// Gives every snippet the dependency `dependency`.
    #[must_use]
    pub fn dependency(mut self, dependency: Dependency) -> Self {
        self.dependencies.push(dependency);
        self
    }

    /// Copies the `Cargo.lock` at `path` into the project before building, so that the dependencies resolve to the versions it holds.
    ///
    /// Cargo still updates the copy where it must, and the original is never written.
    #[must_use]
    pub fn lockfile(mut self, path: impl Into<PathBuf>) -> Self {
        self.lockfile_path = Some(path.into());
        self
    }
}

#[cfg(test)]
mod tests;
