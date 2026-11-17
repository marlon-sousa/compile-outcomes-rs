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
