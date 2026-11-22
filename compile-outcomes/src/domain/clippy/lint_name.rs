//! Entity (clippy): the name of one of clippy's lints, as clippy names it after `clippy::`.

use derive_more::Display;

/// A lint's name with one leading `clippy::` removed, the rest kept as given.
#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub(crate) struct LintName(String);

impl LintName {
    pub(crate) fn given(name: String) -> Self {
        match name.strip_prefix("clippy::") {
            Some(bare) => Self(bare.to_owned()),
            None => Self(name),
        }
    }
}

#[cfg(test)]
mod tests;
