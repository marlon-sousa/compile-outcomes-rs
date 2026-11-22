use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::path::PathBuf;

use super::compilation;
use super::fakes::{FakeCargo, FakeDisk, Step};
use crate::domain::{Error, Tool};
use crate::use_cases::compile::compile;

/// The names of a disk on which `folder` holds only the file `name`.
fn holding(folder: &str, name: &str) -> BTreeMap<PathBuf, BTreeSet<OsString>> {
    BTreeMap::from([(
        PathBuf::from(folder),
        BTreeSet::from([OsString::from(name)]),
    )])
}

/// Cargo that builds the one snippet of [`compilation`] its fake disk gives.
fn built() -> FakeCargo {
    FakeCargo {
        built: vec!["snippet_a".to_owned()],
        ..FakeCargo::default()
    }
}

#[test]
fn a_project_folder_compile_outcomes_did_not_write_is_left_untouched() {
    let disk = FakeDisk {
        names: holding("work/project", "notes.txt"),
        ..FakeDisk::default()
    };
    let cargo = FakeCargo::default();

    let errors = compile(&compilation(), &disk, &cargo).unwrap_err();

    assert_eq!(
        errors.into_iter().collect::<Vec<_>>(),
        [Error::ForeignProjectFolder {
            folder: PathBuf::from("work/project")
        }]
    );
    assert!(disk.written.borrow().is_empty());
    assert!(cargo.builds.borrow().is_empty());
}

#[test]
fn a_project_folder_that_cannot_be_read_is_unwritable_and_nothing_is_built() {
    let disk = FakeDisk {
        unreadable: vec![PathBuf::from("work/project")],
        ..FakeDisk::default()
    };
    let cargo = FakeCargo::default();

    let errors = compile(&compilation(), &disk, &cargo).unwrap_err();

    assert_eq!(
        errors.into_iter().collect::<Vec<_>>(),
        [Error::Unwritable {
            path: PathBuf::from("work/project"),
            reason: "gone".to_owned(),
        }]
    );
    assert!(disk.written.borrow().is_empty());
    assert!(cargo.steps.borrow().is_empty());
}

#[test]
fn a_clean_build_has_cargo_clean_the_build_output_it_made_before_building() {
    let disk = FakeDisk {
        names: holding("work/target", "CACHEDIR.TAG"),
        ..FakeDisk::default()
    };
    let cargo = built();

    let outcomes = compile(&compilation().clean_build(), &disk, &cargo);

    assert!(outcomes.is_ok());
    assert_eq!(
        *cargo.steps.borrow(),
        [Step::Clean, Step::Build(Tool::Build)]
    );
}

#[test]
fn a_build_that_is_not_clean_keeps_the_build_output() {
    let cargo = built();

    let outcomes = compile(&compilation(), &FakeDisk::default(), &cargo);

    assert!(outcomes.is_ok());
    assert_eq!(*cargo.steps.borrow(), [Step::Build(Tool::Build)]);
}

#[test]
fn a_target_folder_cargo_did_not_make_is_left_untouched_by_a_clean_build() {
    let disk = FakeDisk {
        names: holding("work/target", "notes.txt"),
        ..FakeDisk::default()
    };
    let cargo = FakeCargo::default();

    let errors = compile(&compilation().clean_build(), &disk, &cargo).unwrap_err();

    assert_eq!(
        errors.into_iter().collect::<Vec<_>>(),
        [Error::ForeignTargetFolder {
            folder: PathBuf::from("work/target")
        }]
    );
    assert!(disk.written.borrow().is_empty());
    assert!(cargo.steps.borrow().is_empty());
}

#[test]
fn a_target_folder_that_cannot_be_read_is_not_cleaned_and_nothing_is_written() {
    let disk = FakeDisk {
        unreadable: vec![PathBuf::from("work/target")],
        ..FakeDisk::default()
    };
    let cargo = FakeCargo::default();

    let errors = compile(&compilation().clean_build(), &disk, &cargo).unwrap_err();

    assert_eq!(
        errors.into_iter().collect::<Vec<_>>(),
        [Error::TargetFolderNotCleaned {
            folder: PathBuf::from("work/target"),
            reason: "gone".to_owned(),
        }]
    );
    assert!(disk.written.borrow().is_empty());
    assert!(cargo.steps.borrow().is_empty());
}

#[test]
fn a_build_output_cargo_cannot_clean_stops_the_compilation_before_building() {
    let cargo = FakeCargo {
        refuses_cleaning: true,
        ..FakeCargo::default()
    };

    let errors = compile(&compilation().clean_build(), &FakeDisk::default(), &cargo).unwrap_err();

    assert_eq!(
        errors.into_iter().collect::<Vec<_>>(),
        [Error::TargetFolderNotCleaned {
            folder: PathBuf::from("work/target"),
            reason: "busy".to_owned(),
        }]
    );
    assert_eq!(*cargo.steps.borrow(), [Step::Clean]);
}
