//! Entity: one message of the compiler, and where it points.

use std::path::PathBuf;

use derive_more::Display;

/// One message of the compiler about a snippet: an error or a warning.
///
/// It displays as [`rendered`](Self::rendered), the compiler's own text, which changes with the compiler that ran and holds the platform's paths; assertions that last check the other fields.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display("{rendered}")]
#[non_exhaustive]
pub struct Diagnostic {
    /// The first line of the message, such as ``cannot find function `shout` in crate `greeting` ``.
    pub message: String,
    /// The error code or the lint the message comes from, such as `E0425` or `dead_code`, when it has one.
    pub code: Option<String>,
    /// Where the message's first primary span points, when it points anywhere.
    pub location: Option<Location>,
    /// The whole message as a terminal shows it.
    pub rendered: String,
}

/// Where a compiler message points, as the compiler gives it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Location {
    /// The file, as the compiler names it.
    pub file: PathBuf,
    /// The line, counted from 1 as the compiler counts it.
    pub line: usize,
    /// The column, counted from 1 as the compiler counts it.
    pub column: usize,
}
