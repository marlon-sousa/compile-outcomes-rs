use std::path::{Path, PathBuf};

use super::{Build, Compilation, SnippetSource};
use crate::domain::{Dependency, Edition};

#[test]
fn a_new_compilation_has_nothing_but_its_edition_and_folder() {
    let compilation = Compilation::new(Edition::E2021, "target/outcomes");

    assert_eq!(compilation.edition(), Edition::E2021);
    assert_eq!(compilation.work_dir(), Path::new("target/outcomes"));
    assert!(compilation.sources().is_empty());
    assert!(compilation.dependencies().is_empty());
    assert_eq!(compilation.lockfile_path(), &None);
    assert_eq!(compilation.build(), Build::Reusing);
}

#[test]
fn clean_build_asks_for_a_clean_build() {
    let compilation = Compilation::new(Edition::E2021, "work").clean_build();

    assert_eq!(compilation.build(), Build::Clean);
}

#[test]
fn snippets_and_folders_keep_the_order_they_were_given() {
    let compilation = Compilation::new(Edition::E2024, "work")
        .snippet("ui/a.rs")
        .snippets_in("ui/more")
        .snippet("ui/b.rs");

    assert_eq!(
        *compilation.sources(),
        [
            SnippetSource::File(PathBuf::from("ui/a.rs")),
            SnippetSource::Folder(PathBuf::from("ui/more")),
            SnippetSource::File(PathBuf::from("ui/b.rs")),
        ]
    );
}

#[test]
fn dependencies_and_the_lockfile_are_kept() {
    let greeting = Dependency::path("greeting", "fixtures/greeting");
    let compilation = Compilation::new(Edition::E2021, "work")
        .dependency(greeting.clone())
        .lockfile("Cargo.lock");

    assert_eq!(*compilation.dependencies(), [greeting]);
    assert_eq!(
        compilation.lockfile_path().as_deref(),
        Some(Path::new("Cargo.lock"))
    );
}
