//! Error: what keeps a compilation from its outcomes, as a sentence for the user.

mod gathered;

use std::path::PathBuf;

use thiserror::Error;

use crate::domain::{Diagnostic, SnippetName};

pub use gathered::Errors;

/// One problem that kept a compilation from its outcomes.
///
/// Its `Display` is the sentence a user reads.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum Error {
    /// The compilation was given no snippet.
    #[error("The compilation has no snippet. Add one with the snippet or snippets_in method.")]
    NoSnippet,

    /// A snippet's file name breaks the rules of a name.
    #[error(
        "The snippet file {} has no valid name. A name is made of ASCII letters, digits, underscores and hyphens, and does not start with a digit.",
        .file.display()
    )]
    #[non_exhaustive]
    InvalidName {
        /// The snippet's file.
        file: PathBuf,
    },

    /// Two snippets would make the same crate, having the same name or names that differ only by `-` and `_`.
    #[error(
        "The snippet files {} and {} would both make the crate {crate_name}.",
        .first.display(),
        .second.display()
    )]
    #[non_exhaustive]
    DuplicateName {
        /// The crate both would make.
        crate_name: String,
        /// The file found first.
        first: PathBuf,
        /// The file found second.
        second: PathBuf,
    },

    /// A snippet file could not be read.
    #[error("The snippet file {} could not be read: {reason}", .file.display())]
    #[non_exhaustive]
    UnreadableSnippet {
        /// The file.
        file: PathBuf,
        /// The reason the system gave.
        reason: String,
    },

    /// A folder given for its snippets could not be read.
    #[error("The snippet folder {} could not be read: {reason}", .folder.display())]
    #[non_exhaustive]
    UnreadableFolder {
        /// The folder.
        folder: PathBuf,
        /// The reason the system gave.
        reason: String,
    },

    /// The generated project, or the lockfile copied into it, could not be written.
    #[error("The project could not be written at {}: {reason}", .path.display())]
    #[non_exhaustive]
    Unwritable {
        /// The file or folder that could not be written.
        path: PathBuf,
        /// The reason the system gave.
        reason: String,
    },

    /// Cargo could not be started.
    #[error("Cargo, run as {program}, could not be started: {reason}")]
    #[non_exhaustive]
    CargoNotStarted {
        /// The program run, as the `CARGO` environment variable named it, or `cargo`.
        program: String,
        /// The reason the system gave.
        reason: String,
    },

    /// Cargo stopped without building any snippet and without a compiler message.
    #[error("Cargo stopped before compiling any snippet, and wrote: {output}")]
    #[non_exhaustive]
    CargoStopped {
        /// What Cargo wrote to its error output.
        output: String,
    },

    /// A dependency of the snippets failed to compile.
    #[error("The dependency {name} failed to compile. The compiler said:\n{}", messages(.errors))]
    #[non_exhaustive]
    DependencyFailed {
        /// The dependency's crate.
        name: String,
        /// Every error the compiler reported for it, in the order it gave them.
        errors: Vec<Diagnostic>,
    },

    /// Cargo finished without saying whether a snippet compiled.
    #[error("Cargo finished without an outcome for the snippet {name}.")]
    #[non_exhaustive]
    NoOutcome {
        /// The snippet.
        name: SnippetName,
    },
}

fn messages(errors: &[Diagnostic]) -> String {
    errors
        .iter()
        .map(|error| error.message.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests;
