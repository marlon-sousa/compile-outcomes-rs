use super::{Mode, Step, requested, step};
use strum::{IntoEnumIterator, VariantNames};

#[test]
fn ci_or_no_mode_runs_every_mode_in_order() {
    let every = vec![
        Mode::Fmt,
        Mode::Clippy,
        Mode::Arch,
        Mode::Test,
        Mode::Doc,
        Mode::Deny,
        Mode::Package,
    ];
    assert_eq!(requested(Some("ci")), Some(every.clone()));
    assert_eq!(requested(None), Some(every));
}

#[test]
fn a_mode_named_alone_runs_alone() {
    assert_eq!(requested(Some("deny")), Some(vec![Mode::Deny]));
}

#[test]
fn an_unknown_mode_is_refused() {
    assert_eq!(requested(Some("lint")), None);
}

#[test]
fn modes_are_named_as_typed() {
    assert_eq!(
        Mode::VARIANTS,
        ["fmt", "clippy", "arch", "test", "doc", "deny", "package"]
    );
    assert_eq!(Mode::Package.to_string(), "package");
}

#[test]
fn only_the_doc_mode_denies_the_warnings_of_rustdoc() {
    for mode in Mode::iter() {
        assert_eq!(
            matches!(step(mode), Step::Rustdoc(_)),
            mode == Mode::Doc,
            "{mode}"
        );
    }
}

#[test]
fn the_arch_mode_runs_the_architecture_check() {
    assert_eq!(step(Mode::Arch), Step::Architecture);
}
