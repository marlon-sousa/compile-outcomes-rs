//! Entity: the description of a compilation, built step by step.

use std::path::PathBuf;

use getset::{CopyGetters, Getters};
use strum::EnumIs;

use crate::domain::{Clippy, Dependency, Edition};

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
    #[getset(get_copy = "pub(crate)")]
    build: Build,
    #[getset(get = "pub(crate)")]
    tool: Tool,
}

/// Whether a compilation reuses the build output an earlier one left, or removes it first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIs)]
pub(crate) enum Build {
    Reusing,
    Clean,
}

/// Whether a compilation builds the snippets with a plain `cargo build`, or lints them with `cargo clippy`.
#[derive(Debug, Clone, PartialEq, Eq, EnumIs)]
pub(crate) enum Tool {
    Build,
    Clippy(Clippy),
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
            build: Build::Reusing,
            tool: Tool::Build,
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

    /// Runs `cargo clean` on the build output, `work_dir/target`, before building, so that every crate compiles again, dependencies included.
    ///
    /// The target folder is cleaned when it is missing, holds no file at any depth, or holds `CACHEDIR.TAG`, the file Cargo writes into a target folder it makes; otherwise compiling is refused with [`Error::ForeignTargetFolder`](crate::Error::ForeignTargetFolder), and the folder left untouched. Cargo may still refuse to clean it, such as an empty folder from Cargo 1.97 on, which gives [`Error::TargetFolderNotCleaned`](crate::Error::TargetFolderNotCleaned).
    #[must_use]
    pub fn clean_build(mut self) -> Self {
        self.build = Build::Clean;
        self
    }

    /// Compiles the snippets with `cargo clippy` instead of `cargo build`, with the lint levels of `clippy`, so that clippy's lints are reported with the compiler's messages.
    ///
    /// Calling it again replaces the earlier `clippy`. Compiling refuses with [`Error::ClippyNotInstalled`](crate::Error::ClippyNotInstalled) when clippy is not installed for the toolchain.
    #[must_use]
    pub fn clippy(mut self, clippy: Clippy) -> Self {
        self.tool = Tool::Clippy(clippy);
        self
    }
}

#[cfg(test)]
mod tests;
