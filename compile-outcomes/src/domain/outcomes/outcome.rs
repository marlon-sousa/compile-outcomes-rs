//! Entity: what the compiler said about one snippet.

use std::fmt;

use strum::EnumIs;

use crate::domain::Diagnostic;

/// What the compiler said about one snippet.
///
/// It displays as every error the compiler rendered, then every warning, in the compiler's own text: for a failure message, or to compare with a saved file at the cost of updating it whenever the compiler's words or the platform change. Assertions that last check [`errors`](Self::errors) and [`warnings`](Self::warnings). The compiler interleaves errors and warnings by where they point, so a snippet with both displays them in another order than a terminal shows.
#[derive(Debug, Clone, PartialEq, Eq, EnumIs)]
pub enum Outcome {
    /// Cargo produced the snippet's crate.
    Compiled {
        /// Every warning, in the order the compiler gave them.
        warnings: Vec<Diagnostic>,
    },
    /// The compiler reported errors for the snippet.
    Failed {
        /// Every error, in the order the compiler gave them.
        errors: Vec<Diagnostic>,
        /// Every warning, in the order the compiler gave them.
        warnings: Vec<Diagnostic>,
    },
}

impl Outcome {
    /// The errors, none when the snippet compiled.
    pub fn errors(&self) -> &[Diagnostic] {
        match self {
            Outcome::Compiled { .. } => &[],
            Outcome::Failed { errors, .. } => errors,
        }
    }

    /// The warnings, whether the snippet compiled or not.
    pub fn warnings(&self) -> &[Diagnostic] {
        match self {
            Outcome::Compiled { warnings } | Outcome::Failed { warnings, .. } => warnings,
        }
    }
}

/// Written by hand: no derive joins the diagnostics of two lists.
impl fmt::Display for Outcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.errors()
            .iter()
            .chain(self.warnings())
            .try_for_each(|diagnostic| write!(formatter, "{diagnostic}"))
    }
}

#[cfg(test)]
mod tests;
