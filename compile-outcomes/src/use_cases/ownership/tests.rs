use std::collections::BTreeSet;
use std::ffi::OsString;

use super::{project_folder, target_folder};
use crate::domain::{CARGO_MARKER, OWNER_MARKER, OwnedFolder};

fn names(names: &[&str]) -> BTreeSet<OsString> {
    names.iter().map(OsString::from).collect()
}

/// A policy saying what a folder holding the given names is.
type Policy = fn(Option<&BTreeSet<OsString>>) -> OwnedFolder;

#[test]
fn a_folder_is_missing_empty_owned_or_foreign_by_the_names_it_holds() {
    let cases: [(Policy, Option<&[&str]>, OwnedFolder); 8] = [
        (project_folder, None, OwnedFolder::Missing),
        (project_folder, Some(&[]), OwnedFolder::Empty),
        (
            project_folder,
            Some(&["notes.txt", OWNER_MARKER]),
            OwnedFolder::Owned,
        ),
        (
            project_folder,
            Some(&["notes.txt", CARGO_MARKER]),
            OwnedFolder::Foreign,
        ),
        (target_folder, None, OwnedFolder::Missing),
        (target_folder, Some(&[]), OwnedFolder::Empty),
        (
            target_folder,
            Some(&["debug", CARGO_MARKER]),
            OwnedFolder::Owned,
        ),
        (
            target_folder,
            Some(&["debug", OWNER_MARKER]),
            OwnedFolder::Foreign,
        ),
    ];

    for (folder, held, expected) in cases {
        let held = held.map(names);
        assert_eq!(folder(held.as_ref()), expected, "{held:?}");
    }
}
