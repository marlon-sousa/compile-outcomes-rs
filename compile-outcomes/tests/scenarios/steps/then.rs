//! The steps that check what a compilation gave.

use std::env;
use std::fs;

use compile_outcomes::{Diagnostic, Errors, Location, Outcome, Outcomes};
use cucumber::gherkin::Step;
use cucumber::then;
use toml::{Table, Value};

use super::{Compiling, fixture, work_dir};

impl Compiling {
    fn outcomes(&self) -> &Outcomes {
        match &self.result {
            Some(Ok(outcomes)) => outcomes,
            other => panic!("the compilation gave no outcomes: {other:?}"),
        }
    }

    pub(super) fn errors(&self) -> &Errors {
        match &self.result {
            Some(Err(errors)) => errors,
            other => panic!("the compilation was not refused: {other:?}"),
        }
    }

    fn outcome(&self, snippet: &str) -> &Outcome {
        self.outcomes()
            .get(snippet)
            .unwrap_or_else(|| panic!("no outcome for {snippet}"))
    }
}

fn docstring(step: &Step) -> Vec<&str> {
    let text = step.docstring.as_deref().expect("the step has a docstring");
    text.trim().lines().map(str::trim).collect()
}

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

#[then(expr = "the snippet {string} compiled")]
fn compiled(world: &mut Compiling, snippet: String) {
    let outcome = world.outcome(&snippet);
    assert!(outcome.is_compiled(), "{outcome:#?}");
}

#[then(expr = "the snippet {string} failed")]
fn failed(world: &mut Compiling, snippet: String) {
    let outcome = world.outcome(&snippet);
    assert!(outcome.is_failed(), "{outcome:#?}");
}

#[then(expr = "the snippet {string} has no warnings")]
fn no_warnings(world: &mut Compiling, snippet: String) {
    assert_eq!(world.outcome(&snippet).warnings(), []);
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

#[then(expr = "the errors of the snippet {string} are, in order:")]
fn the_errors_in_order(world: &mut Compiling, snippet: String, step: &Step) {
    let messages: Vec<&str> = world
        .outcome(&snippet)
        .errors()
        .iter()
        .map(|error| error.message.as_str())
        .collect();
    assert_eq!(messages, docstring(step));
}

#[then(expr = "the compilation has the snippets {string}")]
fn the_snippets(world: &mut Compiling, names: String) {
    let found: Vec<&str> = world
        .outcomes()
        .iter()
        .map(|(name, _)| name.as_str())
        .collect();
    assert_eq!(found.join(", "), names);
}

/// Each line of the docstring starts the line of the errors at the same place, paths written with `/`.
#[then("the compilation is refused with:")]
fn refused_with(world: &mut Compiling, step: &Step) {
    let said = world.errors().to_string().replace('\\', "/");
    let said: Vec<&str> = said.lines().collect();
    let expected = docstring(step);
    assert!(expected.len() <= said.len(), "{said:#?}");
    for (line, start) in said.iter().zip(&expected) {
        assert!(line.starts_with(start), "{said:#?}");
    }
}

#[then(expr = "the compilation is refused with {int} problem(s)")]
fn refused_with_problems(world: &mut Compiling, count: usize) {
    let errors = world.errors();
    assert_eq!(errors.iter().count(), count, "{errors}");
}

#[then(expr = "the project resolved {string} to the version {string}")]
fn resolved_to(_world: &mut Compiling, name: String, version: String) {
    let lockfile = project_lockfile();
    let mut packages = lockfile
        .get("package")
        .and_then(Value::as_array)
        .into_iter()
        .flatten();
    assert!(
        packages.any(|package| is_locked(package, &name, &version)),
        "{lockfile}"
    );
}

fn is_locked(package: &Value, name: &str, version: &str) -> bool {
    package.get("name").and_then(Value::as_str) == Some(name)
        && package.get("version").and_then(Value::as_str) == Some(version)
}

#[then(expr = "the project's lockfile is of version {int}, as the copy was")]
fn the_lockfile_version(_world: &mut Compiling, version: i64) {
    let lockfile = project_lockfile();
    let locked = lockfile.get("version").and_then(Value::as_integer);
    assert_eq!(locked, Some(version), "{lockfile}");
}

fn project_lockfile() -> Table {
    fs::read_to_string(work_dir().join("project/Cargo.lock"))
        .unwrap()
        .parse()
        .unwrap()
}

#[then("the lockfile \"lockfile.lock\" is left as it was")]
fn left_as_it_was(_world: &mut Compiling) {
    let text = fs::read_to_string(fixture("lockfile.lock")).unwrap();
    assert_eq!(text, include_str!("../../fixtures/lockfile.lock"));
}

#[then(
    expr = "the warning of {string} points at its file as the compiler names it, with the separators of the platform"
)]
fn points_at_its_file(world: &mut Compiling, snippet: String) {
    let warnings = world.outcome(&snippet).warnings();
    let location = warnings
        .first()
        .and_then(|warning| warning.location.as_ref());
    let file = ["tests", "fixtures", "snippets", &format!("{snippet}.rs")]
        .iter()
        .fold(env::current_dir().unwrap(), |path, part| path.join(part));
    let named = location.map(|location| location.file.display().to_string());
    assert_eq!(named, Some(file.display().to_string()));
}
