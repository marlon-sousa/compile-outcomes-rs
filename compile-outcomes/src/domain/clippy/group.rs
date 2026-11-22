//! Entity (clippy): the groups of lints clippy names.

use derive_more::Display;

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

#[cfg(test)]
mod tests;
