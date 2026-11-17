use std::path::{Path, PathBuf};

use toml::Table;

use super::{package, workspace};
use crate::domain::{Dependency, Edition, Package, Project, Resolver};

fn project(dependencies: Vec<Dependency>) -> Project {
    Project {
        root: PathBuf::from("work/project"),
        target_dir: PathBuf::from("work/target"),
        resolver: Resolver::V2,
        edition: Edition::E2018,
        packages: vec![
            Package {
                name: "snippet_a".to_owned(),
                library: PathBuf::from("ui/a.rs"),
            },
            Package {
                name: "snippet_b".to_owned(),
                library: PathBuf::from("ui/b.rs"),
            },
        ],
        dependencies,
        lockfile: None,
    }
}

fn parsed(manifest: &str) -> Table {
    manifest.parse().unwrap()
}

#[test]
fn the_workspace_lists_every_package_with_its_resolver() {
    let expected: Table = r#"
        [workspace]
        members = ["snippet_a", "snippet_b"]
        resolver = "2"
    "#
    .parse()
    .unwrap();

    assert_eq!(parsed(&workspace(&project(Vec::new()))), expected);
}

fn lossy(path: PathBuf) -> String {
    path.to_string_lossy().into_owned()
}

#[test]
fn a_package_is_a_library_rooted_at_its_snippet_made_absolute() {
    let project = project(Vec::new());
    let base = Path::new("base");
    let mut expected: Table = r#"
        [package]
        name = "snippet_a"
        version = "0.0.0"
        edition = "2018"
        publish = false

        [lib]

        [dependencies]
    "#
    .parse()
    .unwrap();
    expected["lib"]
        .as_table_mut()
        .unwrap()
        .insert("path".to_owned(), lossy(base.join("ui/a.rs")).into());

    assert_eq!(
        parsed(&package(&project, &project.packages[0], base)),
        expected
    );
}

#[test]
fn dependencies_are_written_as_cargo_reads_them() {
    let project = project(vec![
        Dependency::path("greeting", "fixtures/greeting").features(["loud"]),
        Dependency::version("serde", "1.0").without_default_features(),
    ]);
    let base = Path::new("base");
    let mut expected: Table = r#"
        greeting = { features = ["loud"] }
        serde = { version = "1.0", default-features = false }
    "#
    .parse()
    .unwrap();
    expected["greeting"].as_table_mut().unwrap().insert(
        "path".to_owned(),
        lossy(base.join("fixtures/greeting")).into(),
    );

    let manifest = parsed(&package(&project, &project.packages[0], base));

    assert_eq!(manifest["dependencies"].as_table(), Some(&expected));
}
