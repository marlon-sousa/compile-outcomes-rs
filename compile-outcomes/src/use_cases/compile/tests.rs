mod clippy;
mod fakes;
mod ownership;

use std::collections::BTreeSet;
use std::path::PathBuf;

use super::compile;
use crate::domain::{Compilation, Edition, Error, Outcome};
use fakes::{FakeCargo, FakeDisk};

fn compilation() -> Compilation {
    Compilation::new(Edition::E2021, "work")
        .snippet("ui/a.rs")
        .snippets_in("ui/more")
}

#[test]
fn snippets_from_files_and_folders_are_written_then_built_into_outcomes() {
    let disk = FakeDisk {
        folder: BTreeSet::from([PathBuf::from("ui/more/b.rs")]),
        ..FakeDisk::default()
    };
    let cargo = FakeCargo {
        built: vec!["snippet_a".to_owned(), "snippet_b".to_owned()],
        ..FakeCargo::default()
    };

    let outcomes = compile(&compilation(), &disk, &cargo).unwrap();

    assert!(outcomes.get("a").is_some_and(Outcome::is_compiled));
    assert!(outcomes.get("b").is_some_and(Outcome::is_compiled));
    assert_eq!(
        disk.written.borrow().as_slice(),
        cargo.builds.borrow().as_slice()
    );
    assert_eq!(disk.written.borrow().len(), 1);
}

#[test]
fn problems_with_the_description_are_reported_together_before_anything_is_written() {
    let disk = FakeDisk {
        unreadable: vec![PathBuf::from("ui/a.rs"), PathBuf::from("gone.lock")],
        folder: BTreeSet::from([PathBuf::from("ui/more/2b.rs")]),
        ..FakeDisk::default()
    };
    let cargo = FakeCargo::default();

    let compilation = compilation().lockfile("gone.lock");

    let errors = compile(&compilation, &disk, &cargo).unwrap_err();

    assert_eq!(
        errors.into_iter().collect::<Vec<_>>(),
        [
            Error::UnreadableSnippet {
                file: PathBuf::from("ui/a.rs"),
                reason: "gone".to_owned(),
            },
            Error::InvalidName {
                file: PathBuf::from("ui/more/2b.rs")
            },
            Error::UnreadableLockfile {
                lockfile: PathBuf::from("gone.lock"),
                reason: "gone".to_owned(),
            },
        ]
    );
    assert!(disk.written.borrow().is_empty());
    assert!(cargo.builds.borrow().is_empty());
}

#[test]
fn a_project_that_cannot_be_written_is_never_built() {
    let disk = FakeDisk {
        refuses_writing: true,
        ..FakeDisk::default()
    };
    let cargo = FakeCargo::default();

    let errors = compile(&compilation(), &disk, &cargo).unwrap_err();

    assert_eq!(
        errors.first(),
        &Error::Unwritable {
            path: PathBuf::from("work/project"),
            reason: "read-only".to_owned(),
        }
    );
    assert!(cargo.builds.borrow().is_empty());
}

#[test]
fn cargo_that_cannot_be_started_is_reported_alone() {
    let disk = FakeDisk::default();
    let cargo = FakeCargo {
        missing: true,
        ..FakeCargo::default()
    };

    let errors = compile(&compilation(), &disk, &cargo).unwrap_err();

    assert_eq!(
        errors.into_iter().collect::<Vec<_>>(),
        [Error::CargoNotStarted {
            program: PathBuf::from("cargo"),
            reason: "No such file or directory".to_owned()
        }]
    );
}

#[test]
fn a_lockfile_that_cannot_be_read_is_a_problem_with_the_description() {
    let disk = FakeDisk {
        unreadable: vec![PathBuf::from("gone.lock")],
        ..FakeDisk::default()
    };
    let cargo = FakeCargo::default();
    let compilation = compilation().lockfile("gone.lock");

    let errors = compile(&compilation, &disk, &cargo).unwrap_err();

    assert_eq!(
        errors.into_iter().collect::<Vec<_>>(),
        [Error::UnreadableLockfile {
            lockfile: PathBuf::from("gone.lock"),
            reason: "gone".to_owned(),
        }]
    );
    assert!(disk.written.borrow().is_empty());
}
