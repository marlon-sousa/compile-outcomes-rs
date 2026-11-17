use std::path::PathBuf;

use super::{Error, Errors};
use crate::test_support::{diagnostic, name};

#[test]
fn every_error_reads_as_a_sentence_naming_what_it_is_about() {
    let cases = [
        (
            Error::NoSnippet,
            "The compilation has no snippet. Add one with the snippet or snippets_in method.",
        ),
        (
            Error::InvalidName {
                file: PathBuf::from("ui/2fields.rs"),
            },
            "The snippet file ui/2fields.rs has no valid name. A name is made of ASCII letters, digits, underscores and hyphens, and does not start with a digit.",
        ),
        (
            Error::DuplicateName {
                crate_name: "snippet_missing_field".to_owned(),
                first: PathBuf::from("ui/missing_field.rs"),
                second: PathBuf::from("ui/missing-field.rs"),
            },
            "The snippet files ui/missing_field.rs and ui/missing-field.rs would both make the crate snippet_missing_field.",
        ),
        (
            Error::UnreadableSnippet {
                file: PathBuf::from("ui/a.rs"),
                reason: "No such file or directory (os error 2)".to_owned(),
            },
            "The snippet file ui/a.rs could not be read: No such file or directory (os error 2)",
        ),
        (
            Error::UnreadableFolder {
                folder: PathBuf::from("ui"),
                reason: "No such file or directory (os error 2)".to_owned(),
            },
            "The snippet folder ui could not be read: No such file or directory (os error 2)",
        ),
        (
            Error::Unwritable {
                path: PathBuf::from("work/project"),
                reason: "Permission denied (os error 13)".to_owned(),
            },
            "The project could not be written at work/project: Permission denied (os error 13)",
        ),
        (
            Error::CargoNotStarted {
                program: "cargo".to_owned(),
                reason: "No such file or directory (os error 2)".to_owned(),
            },
            "Cargo, run as cargo, could not be started: No such file or directory (os error 2)",
        ),
        (
            Error::CargoStopped {
                output: "error: no matching package named `greting` found".to_owned(),
            },
            "Cargo stopped before compiling any snippet, and wrote: error: no matching package named `greting` found",
        ),
        (
            Error::DependencyFailed {
                name: "greeting".to_owned(),
                errors: vec![diagnostic("expected `;`"), diagnostic("unknown field")],
            },
            "The dependency greeting failed to compile. The compiler said:\nexpected `;`\nunknown field",
        ),
        (
            Error::NoOutcome {
                name: name("missing_field"),
            },
            "Cargo finished without an outcome for the snippet missing_field.",
        ),
    ];

    for (error, sentence) in cases {
        assert_eq!(error.to_string(), sentence);
    }
}

#[test]
fn no_errors_gather_into_none() {
    assert_eq!(Errors::gathered(Vec::new()), None);
}

#[test]
fn gathered_errors_keep_the_order_they_were_found() {
    let errors = Errors::gathered(vec![
        Error::NoSnippet,
        Error::CargoNotStarted {
            program: "cargo".to_owned(),
            reason: "gone".to_owned(),
        },
    ])
    .unwrap();

    assert_eq!(errors.first(), &Error::NoSnippet);
    assert_eq!(errors.iter().count(), 2);
    assert_eq!(
        errors.into_iter().last(),
        Some(Error::CargoNotStarted {
            program: "cargo".to_owned(),
            reason: "gone".to_owned()
        })
    );
}

#[test]
fn errors_display_one_sentence_a_line() {
    let errors = Errors::gathered(vec![
        Error::NoSnippet,
        Error::CargoNotStarted {
            program: "cargo".to_owned(),
            reason: "gone".to_owned(),
        },
    ])
    .unwrap();

    assert_eq!(
        errors.to_string(),
        "The compilation has no snippet. Add one with the snippet or snippets_in method.\nCargo, run as cargo, could not be started: gone"
    );
}

#[test]
fn one_error_makes_errors_of_its_own() {
    assert_eq!(
        Errors::from(Error::NoSnippet).to_string(),
        Error::NoSnippet.to_string()
    );
}
