use std::path::PathBuf;

use super::report;
use crate::domain::{CompilerMessage, Level, Location};

/// What Cargo 1.97 wrote building a workspace of two snippets, one with a warning and one with an error.
const MESSAGES: &str = include_str!("messages.jsonl");

fn crate_and_level(message: &CompilerMessage) -> (&str, Level) {
    (message.crate_name.as_str(), message.level)
}

#[test]
fn every_compiler_message_is_reported_with_its_crate_and_level_in_order() {
    let report = report(MESSAGES.as_bytes(), String::new());

    let messages: Vec<(&str, Level)> = report.messages.iter().map(crate_and_level).collect();
    assert_eq!(
        messages,
        [
            ("snippet_good", Level::Warning),
            ("snippet_bad", Level::Error),
            ("snippet_bad", Level::FailureNote),
        ]
    );
}

#[test]
fn a_diagnostic_keeps_its_first_line_its_code_its_location_and_its_rendering() {
    let report = report(MESSAGES.as_bytes(), String::new());
    let error = &report.messages[1].diagnostic;

    assert_eq!(
        error.message,
        "cannot find function `missing` in this scope"
    );
    assert_eq!(error.code.as_deref(), Some("E0425"));
    assert_eq!(
        error.location,
        Some(Location {
            file: PathBuf::from("/work/bad.rs"),
            line: 3,
            column: 5,
        })
    );
    assert!(
        error
            .rendered
            .starts_with("error[E0425]: cannot find function `missing`")
    );
}

#[test]
fn a_message_pointing_nowhere_has_no_location() {
    let report = report(MESSAGES.as_bytes(), String::new());

    assert_eq!(report.messages[2].diagnostic.location, None);
    assert_eq!(report.messages[2].diagnostic.code, None);
}

#[test]
fn every_crate_built_is_reported_and_the_error_output_is_kept() {
    let report = report(MESSAGES.as_bytes(), "warning: something".to_owned());

    assert_eq!(report.built, ["snippet_good"]);
    assert_eq!(report.error_output, "warning: something");
}

#[test]
fn lines_that_are_not_messages_are_left_out() {
    let report = report(b"Compiling snippet_good\n\n", String::new());

    assert!(report.messages.is_empty());
    assert!(report.built.is_empty());
}

#[test]
fn a_line_that_is_not_utf8_is_skipped_and_reading_goes_on() {
    let output = [b"\xff\n".as_slice(), MESSAGES.as_bytes()].concat();

    let report = report(&output, String::new());

    assert_eq!(report.built, ["snippet_good"]);
}
