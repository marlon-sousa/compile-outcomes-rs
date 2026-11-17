//! Entity: what Cargo reported of a build, before anything is decided about it.

use strum::EnumIs;

use crate::domain::Diagnostic;

/// Every compiler message and every crate built, in the order Cargo reported them, and what Cargo wrote to its error output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BuildReport {
    pub(crate) messages: Vec<CompilerMessage>,
    pub(crate) built: Vec<String>,
    pub(crate) error_output: String,
}

/// One message of the compiler, with the crate it is about.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CompilerMessage {
    pub(crate) crate_name: String,
    pub(crate) level: Level,
    pub(crate) diagnostic: Diagnostic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIs)]
pub(crate) enum Level {
    InternalCompilerError,
    Error,
    Warning,
    FailureNote,
    Note,
    Help,
    /// A level a later compiler added.
    Unknown,
}
