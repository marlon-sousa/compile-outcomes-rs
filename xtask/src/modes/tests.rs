use super::{CrateReadme, Mode, Request, Step, requested, step};
use strum::{IntoEnumIterator, VariantNames};

#[test]
fn ci_or_no_mode_runs_every_mode_in_order() {
    let every = vec![
        Mode::Fmt,
        Mode::Clippy,
        Mode::Arch,
        Mode::Readme,
        Mode::Test,
        Mode::Doc,
        Mode::Deny,
        Mode::Package,
    ];
    let request = Some(Request {
        modes: every,
        crate_readme: CrateReadme::Check,
    });
    assert_eq!(requested(&["ci"]), request);
    assert_eq!(requested(&[]), request);
}

#[test]
fn a_mode_named_alone_runs_alone() {
    assert_eq!(
        requested(&["deny"]),
        Some(Request {
            modes: vec![Mode::Deny],
            crate_readme: CrateReadme::Check,
        })
    );
}

#[test]
fn only_the_readme_mode_writes_the_crate_readme() {
    assert_eq!(
        requested(&["readme", "--write"]),
        Some(Request {
            modes: vec![Mode::Readme],
            crate_readme: CrateReadme::Write,
        })
    );
    assert_eq!(
        step(Mode::Readme, CrateReadme::Write),
        Step::Readme(CrateReadme::Write)
    );
}

#[test]
fn an_unknown_mode_or_argument_is_refused() {
    let unknown: [&[&str]; 5] = [
        &["lint"],
        &["ci", "--write"],
        &["fmt", "--write"],
        &["readme", "--wrte"],
        &["readme", "--write", "extra"],
    ];
    for arguments in unknown {
        assert_eq!(requested(arguments), None, "{arguments:?}");
    }
}

#[test]
fn modes_are_named_as_typed() {
    assert_eq!(
        Mode::VARIANTS,
        [
            "fmt", "clippy", "arch", "readme", "test", "doc", "deny", "package"
        ]
    );
    assert_eq!(Mode::Package.to_string(), "package");
}

#[test]
fn only_the_doc_mode_denies_the_warnings_of_rustdoc() {
    for mode in Mode::iter() {
        assert_eq!(
            matches!(step(mode, CrateReadme::Check), Step::Rustdoc(_)),
            mode == Mode::Doc,
            "{mode}"
        );
    }
}

#[test]
fn the_arch_mode_runs_the_architecture_check() {
    assert_eq!(step(Mode::Arch, CrateReadme::Check), Step::Architecture);
}
