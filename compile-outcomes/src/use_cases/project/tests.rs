use std::path::{Path, PathBuf};

use super::project;
use crate::domain::{Compilation, Dependency, Edition, Package, Resolver};
use crate::test_support::snippet;

#[test]
fn the_project_and_its_build_live_in_the_work_folder() {
    let compilation = Compilation::new(Edition::E2021, "work");

    let project = project(&compilation, &[]);

    assert_eq!(project.root, Path::new("work/project"));
    assert_eq!(project.target_dir, Path::new("work/target"));
}

#[test]
fn each_snippet_is_the_library_of_a_package_named_after_its_crate() {
    let compilation = Compilation::new(Edition::E2021, "work");
    let snippets = [
        snippet("missing-field", "ui/missing-field.rs"),
        snippet("apple", "ui/apple.rs"),
    ];

    let project = project(&compilation, &snippets);

    assert_eq!(
        project.packages,
        [
            Package {
                name: "snippet_missing_field".to_owned(),
                library: PathBuf::from("ui/missing-field.rs"),
            },
            Package {
                name: "snippet_apple".to_owned(),
                library: PathBuf::from("ui/apple.rs"),
            },
        ]
    );
}

#[test]
fn every_package_shares_the_edition_the_dependencies_and_the_lockfile() {
    let greeting = Dependency::path("greeting", "fixtures/greeting");
    let compilation = Compilation::new(Edition::E2018, "work")
        .dependency(greeting.clone())
        .lockfile("Cargo.lock");

    let project = project(&compilation, &[]);

    assert_eq!(project.edition, Edition::E2018);
    assert_eq!(project.dependencies, [greeting]);
    assert_eq!(project.lockfile, Some(PathBuf::from("Cargo.lock")));
}

#[test]
fn the_resolver_is_the_editions_default_and_never_the_first() {
    let cases = [
        (Edition::E2015, Resolver::V2),
        (Edition::E2018, Resolver::V2),
        (Edition::E2021, Resolver::V2),
        (Edition::E2024, Resolver::V3),
    ];

    for (edition, resolver) in cases {
        let compilation = Compilation::new(edition, "work");
        assert_eq!(project(&compilation, &[]).resolver, resolver, "{edition}");
    }
}
