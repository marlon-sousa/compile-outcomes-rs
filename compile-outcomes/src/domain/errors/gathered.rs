//! Error: every problem that kept a compilation from its outcomes, never none.

use std::iter::{Chain, Once, once};
use std::slice::Iter;
use std::vec::IntoIter;

use getset::Getters;
use thiserror::Error;

use crate::domain::Error;

/// Every problem that kept a compilation from its outcomes, in the order they were found; never empty.
///
/// Its `Display` is every problem's sentence, each starting on a line of its own.
#[derive(Debug, Clone, PartialEq, Eq, Error, Getters)]
#[error("{}", sentences(self))]
pub struct Errors {
    /// The first problem found.
    #[getset(get = "pub")]
    first: Error,
    rest: Vec<Error>,
}

impl Errors {
    /// The errors in `errors`, or `None` when there are none.
    pub(crate) fn gathered(errors: Vec<Error>) -> Option<Self> {
        let mut errors = errors.into_iter();
        let first = errors.next()?;
        Some(Self {
            first,
            rest: errors.collect(),
        })
    }

    /// These errors, then `error`.
    pub(crate) fn and(mut self, error: Error) -> Self {
        self.rest.push(error);
        self
    }

    /// Every problem, in the order they were found.
    pub fn iter(&self) -> impl Iterator<Item = &Error> {
        self.into_iter()
    }
}

impl From<Error> for Errors {
    fn from(error: Error) -> Self {
        Self {
            first: error,
            rest: Vec::new(),
        }
    }
}

impl IntoIterator for Errors {
    type Item = Error;
    type IntoIter = Chain<Once<Error>, IntoIter<Error>>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.first).chain(self.rest)
    }
}

impl<'a> IntoIterator for &'a Errors {
    type Item = &'a Error;
    type IntoIter = Chain<Once<&'a Error>, Iter<'a, Error>>;

    fn into_iter(self) -> Self::IntoIter {
        once(&self.first).chain(&self.rest)
    }
}

fn sentences(errors: &Errors) -> String {
    errors
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n")
}
