//! Adapter: what Cargo wrote, as `cargo_metadata` reads it, translated into a `BuildReport`.

use std::path::PathBuf;

use cargo_metadata::Message;
use cargo_metadata::diagnostic::{Diagnostic as Written, DiagnosticLevel, DiagnosticSpan};

use crate::domain::{BuildReport, CompilerMessage, Diagnostic, Level, Location};

/// Every compiler message and every crate built that `output`, Cargo's JSON messages, holds, in order, with Cargo's error output.
///
/// A line that is not UTF-8, such as one a procedural macro printed, is skipped.
pub(super) fn report(output: &[u8], error_output: String) -> BuildReport {
    let messages: Vec<Message> = Message::parse_stream(output)
        .filter_map(Result::ok)
        .collect();
    BuildReport {
        messages: messages.iter().filter_map(compiler_message).collect(),
        built: messages.iter().filter_map(built_crate).collect(),
        error_output,
    }
}

fn compiler_message(message: &Message) -> Option<CompilerMessage> {
    let Message::CompilerMessage(message) = message else {
        return None;
    };
    Some(CompilerMessage {
        crate_name: message.target.name.clone(),
        level: level(&message.message.level),
        diagnostic: diagnostic(&message.message),
    })
}

fn built_crate(message: &Message) -> Option<String> {
    let Message::CompilerArtifact(artifact) = message else {
        return None;
    };
    Some(artifact.target.name.clone())
}

fn level(level: &DiagnosticLevel) -> Level {
    match level {
        DiagnosticLevel::Ice => Level::InternalCompilerError,
        DiagnosticLevel::Error => Level::Error,
        DiagnosticLevel::Warning => Level::Warning,
        DiagnosticLevel::FailureNote => Level::FailureNote,
        DiagnosticLevel::Note => Level::Note,
        DiagnosticLevel::Help => Level::Help,
        _ => Level::Unknown,
    }
}

fn diagnostic(written: &Written) -> Diagnostic {
    Diagnostic {
        message: written
            .message
            .lines()
            .next()
            .unwrap_or_default()
            .to_owned(),
        code: written.code.as_ref().map(|code| code.code.clone()),
        location: written
            .spans
            .iter()
            .find(|span| span.is_primary)
            .map(location),
        rendered: written.rendered.clone().unwrap_or_default(),
    }
}

fn location(span: &DiagnosticSpan) -> Location {
    Location {
        file: PathBuf::from(&span.file_name),
        line: span.line_start,
        column: span.column_start,
    }
}

#[cfg(test)]
mod tests;
