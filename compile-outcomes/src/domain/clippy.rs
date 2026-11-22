//! Entity: the lint levels clippy compiles the snippets with.

mod group;
mod lint_name;

use getset::Getters;

pub use group::Group;
pub(crate) use lint_name::LintName;

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

/// The lints a level covers: a group, or one lint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Lints {
    Group(Group),
    One(LintName),
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
        self.set(LintLevel::Allow, Lints::One(LintName::given(name.into())))
    }

    /// Warns on the lint `name`, such as `unwrap_used`, with or without `clippy::` before it.
    #[must_use]
    pub fn warn_lint(self, name: impl Into<String>) -> Self {
        self.set(LintLevel::Warn, Lints::One(LintName::given(name.into())))
    }

    /// Denies the lint `name`, such as `unwrap_used`, with or without `clippy::` before it, so that a snippet it fires on fails.
    #[must_use]
    pub fn deny_lint(self, name: impl Into<String>) -> Self {
        self.set(LintLevel::Deny, Lints::One(LintName::given(name.into())))
    }

    fn set(mut self, level: LintLevel, lints: Lints) -> Self {
        self.settings.push(Setting { level, lints });
        self
    }
}

#[cfg(test)]
mod tests;
