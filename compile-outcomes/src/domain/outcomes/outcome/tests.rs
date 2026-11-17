use super::Outcome;
use crate::test_support::diagnostic;

#[test]
fn a_compiled_snippet_has_warnings_and_no_errors() {
    let outcome = Outcome::Compiled {
        warnings: vec![diagnostic("unused variable")],
    };

    assert!(outcome.errors().is_empty());
    assert_eq!(outcome.warnings(), [diagnostic("unused variable")]);
}

#[test]
fn a_failed_snippet_has_its_errors_and_its_warnings() {
    let outcome = Outcome::Failed {
        errors: vec![diagnostic("missing field")],
        warnings: Vec::new(),
    };

    assert_eq!(outcome.errors(), [diagnostic("missing field")]);
    assert!(outcome.warnings().is_empty());
}

#[test]
fn an_outcome_displays_every_error_then_every_warning_as_rendered() {
    let outcome = Outcome::Failed {
        errors: vec![diagnostic("missing field"), diagnostic("wrong type")],
        warnings: vec![diagnostic("unused variable")],
    };

    assert_eq!(
        outcome.to_string(),
        "error: missing fielderror: wrong typeerror: unused variable"
    );
}

#[test]
fn a_diagnostic_displays_as_rendered() {
    assert_eq!(
        diagnostic("missing field").to_string(),
        "error: missing field"
    );
}
