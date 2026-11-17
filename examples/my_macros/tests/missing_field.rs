use compile_outcomes::{Compilation, Dependency, Edition, Errors, compile};

#[test]
fn a_struct_without_a_name_is_refused() -> Result<(), Errors> {
    let work_dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("missing_field");
    let compilation = Compilation::new(Edition::E2021, work_dir)
        .snippets_in("tests/ui")
        .dependency(Dependency::path("my_macros", "."));

    let outcomes = compile(&compilation)?;

    let outcome = outcomes
        .get("missing_field")
        .expect("tests/ui/missing_field.rs");
    assert!(outcome.is_failed());
    let error = &outcome.errors()[0];
    assert_eq!(
        error.message,
        "missing field `name` in initializer of `User`"
    );
    assert_eq!(error.code.as_deref(), Some("E0063"));
    assert_eq!(
        error.location.as_ref().map(|location| location.line),
        Some(4)
    );
    Ok(())
}
