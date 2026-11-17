use std::collections::BTreeSet;
use std::ffi::OsString;

use super::project_folder;
use crate::domain::{OWNER_MARKER, ProjectFolder};

fn names(names: &[&str]) -> BTreeSet<OsString> {
    names.iter().map(OsString::from).collect()
}

#[test]
fn no_folder_is_missing() {
    assert_eq!(project_folder(None), ProjectFolder::Missing);
}

#[test]
fn a_folder_holding_nothing_is_empty() {
    assert_eq!(project_folder(Some(&names(&[]))), ProjectFolder::Empty);
}

#[test]
fn a_folder_holding_the_owner_marker_is_owned() {
    let held = names(&["notes.txt", OWNER_MARKER]);

    assert_eq!(project_folder(Some(&held)), ProjectFolder::Owned);
}

#[test]
fn a_folder_holding_files_without_the_owner_marker_is_foreign() {
    let held = names(&["notes.txt"]);

    assert_eq!(project_folder(Some(&held)), ProjectFolder::Foreign);
}
