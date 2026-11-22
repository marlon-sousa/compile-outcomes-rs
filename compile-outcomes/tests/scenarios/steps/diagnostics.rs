//! The steps that check the warnings and errors of a snippet: their message, their code and where they point.

use compile_outcomes::{Diagnostic, Location};
use cucumber::then;

use super::Compiling;

fn line_and_column(location: &Location) -> (usize, usize) {
    (location.line, location.column)
}

/// Whether `diagnostic` says `message`, with `code`, pointing at `line` and `column`.
fn says(diagnostic: &Diagnostic, message: &str, code: &str, line: usize, column: usize) -> bool {
    let at = diagnostic.location.as_ref().map(line_and_column);
    diagnostic.message == message
        && diagnostic.code.as_deref() == Some(code)
        && at == Some((line, column))
}

#[then(
    expr = "the snippet {string} has the warning {string} with the code {string} at line {int}, column {int}"
)]
fn the_warning(
    world: &mut Compiling,
    snippet: String,
    message: String,
    code: String,
    line: usize,
    column: usize,
) {
    let warnings = world.outcome(&snippet).warnings();
    let found = warnings
        .iter()
        .any(|warning| says(warning, &message, &code, line, column));
    assert!(found, "{warnings:#?}");
}

#[then(expr = "the snippet {string} has the warning {string} with the code {string}")]
fn the_warning_anywhere(world: &mut Compiling, snippet: String, message: String, code: String) {
    let warnings = world.outcome(&snippet).warnings();
    let found = warnings.iter().any(|warning| {
        warning.message == message && warning.code.as_deref() == Some(code.as_str())
    });
    assert!(found, "{warnings:#?}");
}

#[then(
    expr = "the snippet {string} has the error {string} with the code {string} at line {int}, column {int}"
)]
fn the_error(
    world: &mut Compiling,
    snippet: String,
    message: String,
    code: String,
    line: usize,
    column: usize,
) {
    let errors = world.outcome(&snippet).errors();
    let found = errors
        .iter()
        .any(|error| says(error, &message, &code, line, column));
    assert!(found, "{errors:#?}");
}
