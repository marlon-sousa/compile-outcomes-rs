//! Entity: the lint levels clippy compiles the snippets with.

use derive_more::Display;
use getset::Getters;

/// The lint levels clippy compiles every snippet with, built step by step and given to [`Compilation::clippy`](crate::Compilation::clippy).
///
/// `Clippy::new()` alone keeps clippy's own levels. Each later level overrides an earlier one for the lints both cover.
///
/// ```
/// use compile_outcomes::{Clippy, Compilation, Edition, Group};
///
/// let compilation = Compilation::new(Edition::E2021, "target/snippets")
///     .snippets_in("tests/ui")
///     .clippy(Clippy::new().warn(Group::Pedantic).deny_lint("unwrap_used"));
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Getters)]
pub struct Clippy {
    #[getset(get = "pub(crate)")]
    settings: Vec<Setting>,
}

/// A group of clippy's lints, as clippy names it after `clippy::`.
///
/// Displayed as clippy names it, such as `pedantic`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Group {
    /// `clippy::all`, every lint clippy warns on or denies by default.
    All,
    /// `clippy::correctness`, code that is outright wrong.
    Correctness,
    /// `clippy::suspicious`, code that is most likely wrong.
    Suspicious,
    /// `clippy::style`, code that could be written more idiomatically.
    Style,
    /// `clippy::complexity`, code that does something simple in a complex way.
    Complexity,
    /// `clippy::perf`, code that could run faster.
    Perf,
    /// `clippy::pedantic`, strict lints, with occasional false positives.
    Pedantic,
    /// `clippy::restriction`, lints that forbid language features, for some code bases.
    Restriction,
    /// `clippy::nursery`, lints still being developed.
    Nursery,
    /// `clippy::cargo`, lints about the package's manifest.
    Cargo,
}

/// One setting of a [`Clippy`]: whether the lints it covers are allowed, warned on or denied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Setting {
    pub(crate) level: LintLevel,
    pub(crate) lints: Lints,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LintLevel {
    Allow,
    Warn,
    Deny,
}

/// The lints a level covers: a group, or one lint by its name without `clippy::`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Lints {
    Group(Group),
    One(String),
}

impl Clippy {
    /// Keeps clippy's own lint levels, those its documentation gives each lint.
    pub fn new() -> Self {
        Self::default()
    }

    /// Allows every lint of `group`.
    #[must_use]
    pub fn allow(self, group: Group) -> Self {
        self.set(LintLevel::Allow, Lints::Group(group))
    }

    /// Warns on every lint of `group`.
    #[must_use]
    pub fn warn(self, group: Group) -> Self {
        self.set(LintLevel::Warn, Lints::Group(group))
    }

    /// Denies every lint of `group`, so that a snippet it fires on fails.
    #[must_use]
    pub fn deny(self, group: Group) -> Self {
        self.set(LintLevel::Deny, Lints::Group(group))
    }

    /// Allows the lint `name`, such as `unwrap_used`, with or without `clippy::` before it.
    #[must_use]
    pub fn allow_lint(self, name: impl Into<String>) -> Self {
        self.set(LintLevel::Allow, one(name.into()))
    }

    /// Warns on the lint `name`, such as `unwrap_used`, with or without `clippy::` before it.
    #[must_use]
    pub fn warn_lint(self, name: impl Into<String>) -> Self {
        self.set(LintLevel::Warn, one(name.into()))
    }

    /// Denies the lint `name`, such as `unwrap_used`, with or without `clippy::` before it, so that a snippet it fires on fails.
    #[must_use]
    pub fn deny_lint(self, name: impl Into<String>) -> Self {
        self.set(LintLevel::Deny, one(name.into()))
    }

    fn set(mut self, level: LintLevel, lints: Lints) -> Self {
        self.settings.push(Setting { level, lints });
        self
    }
}

fn one(name: String) -> Lints {
    match name.strip_prefix("clippy::") {
        Some(bare) => Lints::One(bare.to_owned()),
        None => Lints::One(name),
    }
}

#[cfg(test)]
mod tests;
