//! Entity: the Cargo workspace that compiles a compilation's snippets.

use std::path::PathBuf;

use derive_more::Display;

use crate::domain::{Dependency, Edition};

/// The generated workspace: one package for each snippet, all with the same edition and dependencies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Project {
    pub(crate) root: PathBuf,
    pub(crate) target_dir: PathBuf,
    pub(crate) resolver: Resolver,
    pub(crate) edition: Edition,
    pub(crate) packages: Vec<Package>,
    pub(crate) dependencies: Vec<Dependency>,
    pub(crate) lockfile: Option<PathBuf>,
}

/// A package of the workspace, in the folder of its name under the root, whose library is a snippet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Package {
    pub(crate) name: String,
    pub(crate) library: PathBuf,
}

/// The dependency resolver of the workspace, displayed as a manifest writes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub(crate) enum Resolver {
    #[display("2")]
    V2,
    #[display("3")]
    V3,
}
