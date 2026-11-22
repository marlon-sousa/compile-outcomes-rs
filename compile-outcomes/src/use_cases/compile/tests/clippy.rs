use std::path::PathBuf;

use super::compilation;
use super::fakes::{FakeCargo, FakeDisk, Step};
use crate::domain::{Clippy, Error, Group, Tool};
use crate::use_cases::compile::compile;

#[test]
fn clippy_is_checked_before_the_project_is_written_then_builds_the_project() {
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
