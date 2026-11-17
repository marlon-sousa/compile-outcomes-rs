//! Entity: what the compiler said about every snippet of a compilation.

mod outcome;

use std::collections::BTreeMap;

use derive_more::IntoIterator;

use crate::domain::SnippetName;

pub use outcome::Outcome;

/// The outcome of every snippet of a compilation, in the order of their names.
#[derive(Debug, Clone, PartialEq, Eq, IntoIterator)]
#[into_iterator(owned, ref)]
pub struct Outcomes(BTreeMap<SnippetName, Outcome>);

impl Outcomes {
    pub(crate) fn of(outcomes: BTreeMap<SnippetName, Outcome>) -> Self {
        Self(outcomes)
    }

    /// The outcome of the snippet named `name`, or `None` when the compilation had no such snippet.
    pub fn get(&self, name: &str) -> Option<&Outcome> {
        self.0.get(name)
    }

    /// Every snippet's name and outcome, in the order of the names.
    pub fn iter(&self) -> impl Iterator<Item = (&SnippetName, &Outcome)> {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests;
