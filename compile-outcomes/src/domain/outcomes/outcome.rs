//! Entity: what the compiler said about one snippet.

use strum::EnumIs;

use crate::domain::Diagnostic;

/// What the compiler said about one snippet.
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

#[cfg(test)]
mod tests;
