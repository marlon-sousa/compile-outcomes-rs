use std::path::PathBuf;

use super::outcomes;
use crate::domain::{
    BuildReport, CompilerMessage, Diagnostic, Error, Level, Location, Outcome, Snippet,
};
use crate::test_support::{diagnostic, name, snippet};

fn message(crate_name: &str, level: Level, text: &str) -> CompilerMessage {
    CompilerMessage {
        crate_name: crate_name.to_owned(),
        level,
        diagnostic: diagnostic(text),
    }
}

fn report(messages: Vec<CompilerMessage>, built: &[&str]) -> BuildReport {
    BuildReport {
        messages,
        built: built.iter().map(ToString::to_string).collect(),
        error_output: String::new(),
    }
}

fn two_snippets() -> [Snippet; 2] {
    [snippet("good", "ui/good.rs"), snippet("bad", "ui/bad.rs")]
}

#[test]
fn a_snippet_built_without_errors_compiled_with_its_warnings() {
    let report = report(
        vec![message("snippet_good", Level::Warning, "unused variable")],
        &["greeting", "snippet_good", "snippet_bad"],
    );

    let outcomes = outcomes(&two_snippets(), report).unwrap();

    assert_eq!(
        outcomes.get("good"),
        Some(&Outcome::Compiled {
            warnings: vec![diagnostic("unused variable")]
        })
    );
    assert_eq!(
        outcomes.get("bad"),
        Some(&Outcome::Compiled { warnings: vec![] })
    );
}

#[test]
fn a_snippet_with_errors_failed_with_its_errors_and_warnings_in_order() {
    let report = report(
        vec![
            message("snippet_bad", Level::Error, "first"),
            message("snippet_bad", Level::Warning, "careful"),
            message("snippet_bad", Level::InternalCompilerError, "second"),
            message("snippet_bad", Level::FailureNote, "try rustc --explain"),
            message("snippet_bad", Level::Note, "a note"),
            message("snippet_bad", Level::Help, "some help"),
            message("snippet_bad", Level::Unknown, "a later level"),
        ],
        &["snippet_good"],
    );

    let outcomes = outcomes(&two_snippets(), report).unwrap();

    assert_eq!(
        outcomes.get("bad"),
        Some(&Outcome::Failed {
            errors: vec![diagnostic("first"), diagnostic("second")],
            warnings: vec![diagnostic("careful")],
        })
    );
}

#[test]
fn a_dependency_with_errors_stops_the_compilation_naming_it() {
    let report = report(
        vec![
            message("greeting", Level::Warning, "unused import"),
            message("greeting", Level::Error, "expected `;`"),
            message("snippet_bad", Level::Error, "unresolved import"),
            message("greeting", Level::Error, "unknown field"),
        ],
        &[],
    );

    assert_eq!(
        outcomes(&two_snippets(), report),
        Err(Error::DependencyFailed {
            name: "greeting".to_owned(),
            errors: vec![diagnostic("expected `;`"), diagnostic("unknown field")],
        })
    );
}

#[test]
fn cargo_stopped_early_only_without_a_message_and_without_a_snippet_built() {
    let cases = [
        (Vec::new(), vec!["greeting"], true),
        (Vec::new(), vec!["snippet_good", "snippet_bad"], false),
        (
            vec![message("snippet_bad", Level::Error, "missing")],
            vec![],
            false,
        ),
    ];

    for (messages, built, stopped) in cases {
        let result = outcomes(&two_snippets(), report(messages, &built));
        assert_eq!(
            matches!(result, Err(Error::CargoStopped { .. })),
            stopped,
            "{result:?}"
        );
    }
}

#[test]
fn snippets_built_without_any_message_all_compiled() {
    let report = report(Vec::new(), &["snippet_good", "snippet_bad"]);

    let outcomes = outcomes(&two_snippets(), report).unwrap();

    let compiled = Outcome::Compiled {
        warnings: Vec::new(),
    };
    assert_eq!(outcomes.get("good"), Some(&compiled));
    assert_eq!(outcomes.get("bad"), Some(&compiled));
}

#[test]
fn cargo_stopping_before_any_snippet_carries_what_it_wrote() {
    let mut report = report(Vec::new(), &["greeting"]);
    report.error_output = "error: no matching package named `greting` found".to_owned();

    assert_eq!(
        outcomes(&two_snippets(), report),
        Err(Error::CargoStopped {
            output: "error: no matching package named `greting` found".to_owned()
        })
    );
}

#[test]
fn a_snippet_without_its_crate_and_without_errors_has_no_outcome() {
    let report = report(
        vec![message("snippet_good", Level::Warning, "w")],
        &["snippet_good"],
    );

    assert_eq!(
        outcomes(&two_snippets(), report),
        Err(Error::NoOutcome { name: name("bad") })
    );
}

#[test]
fn diagnostics_keep_their_code_location_and_rendering() {
    let mut located = message("snippet_bad", Level::Error, "missing field");
    located.diagnostic = Diagnostic {
        code: Some("E0063".to_owned()),
        location: Some(Location {
            file: PathBuf::from("ui/bad.rs"),
            line: 4,
            column: 5,
        }),
        rendered: "error[E0063]: missing field".to_owned(),
        ..located.diagnostic
    };
    let report = report(vec![located.clone()], &["snippet_good"]);

    let outcomes = outcomes(&two_snippets(), report).unwrap();

    assert_eq!(
        outcomes.get("bad").map(Outcome::errors),
        Some([located.diagnostic].as_slice())
    );
}
