use std::path::PathBuf;

use super::compilation;
use super::fakes::{FakeCargo, FakeDisk, Step};
use crate::domain::{Clippy, Error, Group, Tool};
use crate::use_cases::compile::compile;

#[test]
fn a_compilation_with_clippy_checks_for_it_then_builds_with_it() {
    let disk = FakeDisk::default();
    let cargo = FakeCargo {
        built: vec!["snippet_a".to_owned()],
        ..FakeCargo::default()
    };
    let clippy = Clippy::new().warn(Group::Pedantic);

    let compilation = compilation().clippy(clippy.clone());
    compile(&compilation, &disk, &cargo).unwrap();

    assert_eq!(
        *cargo.steps.borrow(),
        [Step::CheckClippy, Step::Build(Tool::Clippy(clippy))]
    );
}

#[test]
fn a_compilation_without_clippy_never_checks_for_it() {
    let disk = FakeDisk::default();
    let cargo = FakeCargo {
        without_clippy: true,
        built: vec!["snippet_a".to_owned()],
        ..FakeCargo::default()
    };

    compile(&compilation(), &disk, &cargo).unwrap();

    assert_eq!(*cargo.steps.borrow(), [Step::Build(Tool::Build)]);
}

#[test]
fn clippy_that_is_not_installed_is_reported_alone_before_anything_is_written_or_cleaned() {
    let disk = FakeDisk::default();
    let cargo = FakeCargo {
        without_clippy: true,
        ..FakeCargo::default()
    };

    let compilation = compilation().clean_build().clippy(Clippy::new());
    let errors = compile(&compilation, &disk, &cargo).unwrap_err();

    assert_eq!(
        errors.into_iter().collect::<Vec<_>>(),
        [Error::ClippyNotInstalled {
            program: PathBuf::from("cargo"),
            output: "no such command: `clippy`".to_owned(),
        }]
    );
    assert!(disk.written.borrow().is_empty());
    assert_eq!(*cargo.steps.borrow(), [Step::CheckClippy]);
}

#[test]
fn a_clean_build_with_clippy_checks_for_it_then_cleans_then_builds_with_it() {
    let disk = FakeDisk::default();
    let cargo = FakeCargo {
        built: vec!["snippet_a".to_owned()],
        ..FakeCargo::default()
    };

    let compilation = compilation().clean_build().clippy(Clippy::new());
    compile(&compilation, &disk, &cargo).unwrap();

    assert_eq!(
        *cargo.steps.borrow(),
        [
            Step::CheckClippy,
            Step::Clean,
            Step::Build(Tool::Clippy(Clippy::new()))
        ]
    );
}
