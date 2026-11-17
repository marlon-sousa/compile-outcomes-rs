//! Entity: a crate every snippet of a compilation depends on.

use std::path::PathBuf;

use getset::{CopyGetters, Getters};
use strum::EnumIs;

/// A crate every snippet of a compilation depends on, as a manifest's `[dependencies]` names it.
#[derive(Debug, Clone, PartialEq, Eq, Getters, CopyGetters)]
pub struct Dependency {
    #[getset(get = "pub(crate)")]
    name: String,
    #[getset(get = "pub(crate)")]
    source: Source,
    #[getset(get = "pub(crate)")]
    enabled_features: Vec<String>,
    #[getset(get_copy = "pub(crate)")]
    default_features: DefaultFeatures,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Source {
    Path(PathBuf),
    Version(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIs)]
pub(crate) enum DefaultFeatures {
    On,
    Off,
}

impl Dependency {
    /// The crate `name` in the folder `path`, as `name = { path = "..." }` in a manifest.
    pub fn path(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self::of(name.into(), Source::Path(path.into()))
    }

    /// The crate `name` from crates.io, as `name = "requirement"` in a manifest.
    pub fn version(name: impl Into<String>, requirement: impl Into<String>) -> Self {
        Self::of(name.into(), Source::Version(requirement.into()))
    }

    /// Enables these features of the crate, besides any enabled before.
    #[must_use]
    pub fn features<I>(mut self, features: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        self.enabled_features
            .extend(features.into_iter().map(Into::into));
        self
    }

    /// Turns the crate's default features off.
    #[must_use]
    pub fn without_default_features(mut self) -> Self {
        self.default_features = DefaultFeatures::Off;
        self
    }

    fn of(name: String, source: Source) -> Self {
        Self {
            name,
            source,
            enabled_features: Vec::new(),
            default_features: DefaultFeatures::On,
        }
    }
}

#[cfg(test)]
mod tests;
