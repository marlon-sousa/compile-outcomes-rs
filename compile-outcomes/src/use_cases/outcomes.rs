//! Policy: the outcome of every snippet, from what Cargo reported, or the error that kept the compilation from them.

use std::collections::{BTreeMap, BTreeSet};

use crate::domain::{
    BuildReport, CompilerMessage, Diagnostic, Error, Level, Outcome, Outcomes, Snippet, SnippetName,
};

/// The outcome of every snippet, or the first problem that kept Cargo from giving them.
pub(crate) fn outcomes(snippets: &[Snippet], report: BuildReport) -> Result<Outcomes, Error> {
    let crates: BTreeSet<String> = snippets.iter().map(Snippet::crate_name).collect();
    if stopped_early(&crates, &report) {
        return Err(Error::CargoStopped {
            output: report.error_output,
        });
    }
    if let Some(error) = dependency_failure(&crates, &report.messages) {
        return Err(error);
    }
    snippets
        .iter()
        .map(|snippet| outcome(snippet, &report))
        .collect::<Result<BTreeMap<_, _>, _>>()
        .map(Outcomes::of)
}

fn stopped_early(crates: &BTreeSet<String>, report: &BuildReport) -> bool {
    let built_a_snippet = report.built.iter().any(|built| crates.contains(built));
    report.messages.is_empty() && !built_a_snippet
}

/// The failure of the first crate other than a snippet's that the compiler reported an error for, `None` when every error is a snippet's.
fn dependency_failure(crates: &BTreeSet<String>, messages: &[CompilerMessage]) -> Option<Error> {
    let failed = messages
        .iter()
        .filter(|message| is_error(&message.level))
        .map(|message| &message.crate_name)
        .find(|name| is_dependency(name, crates))?;
    Some(Error::DependencyFailed {
        name: failed.clone(),
        errors: diagnostics(messages, failed, is_error),
    })
}

fn is_dependency(crate_name: &str, snippet_crates: &BTreeSet<String>) -> bool {
    !snippet_crates.contains(crate_name)
}

fn outcome(snippet: &Snippet, report: &BuildReport) -> Result<(SnippetName, Outcome), Error> {
    let crate_name = snippet.crate_name();
    let errors = diagnostics(&report.messages, &crate_name, is_error);
    let warnings = diagnostics(&report.messages, &crate_name, Level::is_warning);
    let outcome = if !errors.is_empty() {
        Outcome::Failed { errors, warnings }
    } else if report.built.contains(&crate_name) {
        Outcome::Compiled { warnings }
    } else {
        return Err(Error::NoOutcome {
            name: snippet.name.clone(),
        });
    };
    Ok((snippet.name.clone(), outcome))
}

fn diagnostics(
    messages: &[CompilerMessage],
    crate_name: &str,
    keeps: fn(&Level) -> bool,
) -> Vec<Diagnostic> {
    messages
        .iter()
        .filter(|message| message.crate_name == crate_name)
        .filter(|message| keeps(&message.level))
        .map(|message| message.diagnostic.clone())
        .collect()
}

fn is_error(level: &Level) -> bool {
    matches!(level, Level::Error | Level::InternalCompilerError)
}

#[cfg(test)]
mod tests;
