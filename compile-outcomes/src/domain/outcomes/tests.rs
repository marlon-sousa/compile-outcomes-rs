use std::collections::BTreeMap;

use super::{Outcome, Outcomes};
use crate::test_support::{diagnostic, name};

fn outcomes() -> Outcomes {
    Outcomes::of(BTreeMap::from([
        (
            name("zebra"),
            Outcome::Compiled {
                warnings: vec![diagnostic("unused variable")],
            },
        ),
        (
            name("apple"),
            Outcome::Failed {
                errors: vec![diagnostic("missing field")],
                warnings: Vec::new(),
            },
        ),
    ]))
}

#[test]
fn an_outcome_is_looked_up_by_its_snippet_name() {
    let outcomes = outcomes();

    assert!(outcomes.get("apple").is_some_and(Outcome::is_failed));
    assert!(outcomes.get("zebra").is_some_and(Outcome::is_compiled));
    assert_eq!(outcomes.get("pear"), None);
}

#[test]
fn outcomes_are_iterated_in_the_order_of_the_names() {
    let outcomes = outcomes();
    let names: Vec<&str> = outcomes.iter().map(|(name, _)| name.as_str()).collect();

    assert_eq!(names, ["apple", "zebra"]);
}

#[test]
fn a_compiled_snippet_has_warnings_and_no_errors() {
    let outcomes = outcomes();
    let zebra = outcomes.get("zebra").unwrap();

    assert!(zebra.errors().is_empty());
    assert_eq!(zebra.warnings(), [diagnostic("unused variable")]);
}

#[test]
fn a_failed_snippet_has_its_errors_and_its_warnings() {
    let outcomes = outcomes();
    let apple = outcomes.get("apple").unwrap();

    assert_eq!(apple.errors(), [diagnostic("missing field")]);
    assert!(apple.warnings().is_empty());
}
