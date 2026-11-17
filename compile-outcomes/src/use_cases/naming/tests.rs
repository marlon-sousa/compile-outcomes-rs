use std::path::PathBuf;

use super::snippets;
use crate::domain::{Error, Found};
use crate::test_support::snippet;

fn file(path: &str) -> Found {
    Found::File(PathBuf::from(path))
}

fn folder(files: &[&str]) -> Found {
    Found::Folder(files.iter().map(PathBuf::from).collect())
}

fn unreadable_folder(path: &str) -> Found {
    Found::UnreadableFolder {
        folder: PathBuf::from(path),
        reason: "No such file or directory".to_owned(),
    }
}

fn errors_of(found: Vec<Found>) -> Vec<Error> {
    snippets(found).unwrap_err().into_iter().collect()
}

#[test]
fn snippets_are_named_after_their_files_in_the_order_found() {
    let found = vec![
        file("ui/zebra.rs"),
        folder(&["ui/more/missing-field.rs", "ui/more/apple.rs"]),
    ];

    assert_eq!(
        snippets(found).unwrap(),
        [
            snippet("zebra", "ui/zebra.rs"),
            snippet("apple", "ui/more/apple.rs"),
            snippet("missing-field", "ui/more/missing-field.rs"),
        ]
    );
}

#[test]
fn a_folder_gives_only_its_rust_files() {
    let found = vec![folder(&["ui/a.rs", "ui/a.stderr", "ui/README", "ui/b.RS"])];

    assert_eq!(snippets(found).unwrap(), [snippet("a", "ui/a.rs")]);
}

#[test]
fn a_file_given_alone_is_a_snippet_whatever_its_extension() {
    let cases = [
        ("ui/case.txt", "case"),
        ("ui/notes", "notes"),
        ("ui/case.rs", "case"),
    ];

    for (path, name) in cases {
        assert_eq!(snippets(vec![file(path)]).unwrap(), [snippet(name, path)]);
    }
}

#[test]
fn a_compilation_whose_places_hold_no_file_has_no_snippet() {
    let cases = [
        Vec::new(),
        vec![folder(&["ui/notes.txt"])],
        vec![folder(&[])],
    ];

    for found in cases {
        assert_eq!(errors_of(found), [Error::NoSnippet]);
    }
}

#[test]
fn a_place_that_could_not_be_read_is_reported_instead_of_no_snippet() {
    let cases = [
        (
            unreadable_folder("ui"),
            Error::UnreadableFolder {
                folder: PathBuf::from("ui"),
                reason: "No such file or directory".to_owned(),
            },
        ),
        (
            Found::UnreadableSnippet {
                file: PathBuf::from("ui/lost.rs"),
                reason: "Permission denied".to_owned(),
            },
            Error::UnreadableSnippet {
                file: PathBuf::from("ui/lost.rs"),
                reason: "Permission denied".to_owned(),
            },
        ),
    ];

    for (found, error) in cases {
        assert_eq!(errors_of(vec![found]), [error]);
    }
}

#[test]
fn a_file_whose_name_without_its_extension_breaks_the_rules_is_refused_naming_it() {
    let cases = ["ui/2fields.rs", "ui/a.b.rs", "ui/.rs", "ui/with space.rs"];

    for path in cases {
        assert_eq!(
            errors_of(vec![file(path)]),
            [Error::InvalidName {
                file: PathBuf::from(path)
            }],
            "{path}"
        );
    }
}

#[test]
fn snippets_whose_crates_would_clash_are_refused_naming_both_files() {
    let found = vec![folder(&["ui/missing_field.rs", "ui/missing-field.rs"])];

    assert_eq!(
        errors_of(found),
        [Error::DuplicateName {
            crate_name: "snippet_missing_field".to_owned(),
            first: PathBuf::from("ui/missing-field.rs"),
            second: PathBuf::from("ui/missing_field.rs"),
        }]
    );
}

#[test]
fn every_problem_is_reported_together_in_the_order_found() {
    let found = vec![
        unreadable_folder("ui/gone"),
        Found::UnreadableSnippet {
            file: PathBuf::from("ui/lost.rs"),
            reason: "Permission denied".to_owned(),
        },
        file("ui/a.rs"),
        folder(&["ui/2b.rs", "ui/a.rs"]),
    ];

    assert_eq!(
        errors_of(found),
        [
            Error::UnreadableFolder {
                folder: PathBuf::from("ui/gone"),
                reason: "No such file or directory".to_owned(),
            },
            Error::UnreadableSnippet {
                file: PathBuf::from("ui/lost.rs"),
                reason: "Permission denied".to_owned(),
            },
            Error::InvalidName {
                file: PathBuf::from("ui/2b.rs")
            },
            Error::DuplicateName {
                crate_name: "snippet_a".to_owned(),
                first: PathBuf::from("ui/a.rs"),
                second: PathBuf::from("ui/a.rs"),
            },
        ]
    );
}
