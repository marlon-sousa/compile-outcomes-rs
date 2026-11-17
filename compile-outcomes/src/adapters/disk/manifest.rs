//! Adapter: the manifests of the project, written with `toml`, every path in them made absolute from `base`.

use std::path::Path;

use toml::{Table, Value};

use crate::domain::{Dependency, Package, Project, Source};

/// The manifest at the project's root: a workspace of every package.
pub(super) fn workspace(project: &Project) -> String {
    let members: Vec<Value> = project
        .packages
        .iter()
        .map(|package| package.name.as_str())
        .map(Value::from)
        .collect();
    let mut workspace = Table::new();
    workspace.insert("members".to_owned(), Value::from(members));
    workspace.insert(
        "resolver".to_owned(),
        Value::from(project.resolver.to_string()),
    );
    let mut manifest = Table::new();
    manifest.insert("workspace".to_owned(), Value::from(workspace));
    manifest.to_string()
}

/// The manifest of `package`: a library whose root is its snippet, with every dependency of the project.
pub(super) fn package(project: &Project, package: &Package, base: &Path) -> String {
    let mut about = Table::new();
    about.insert("name".to_owned(), Value::from(package.name.as_str()));
    about.insert("version".to_owned(), Value::from("0.0.0"));
    about.insert(
        "edition".to_owned(),
        Value::from(project.edition.to_string()),
    );
    about.insert("publish".to_owned(), Value::from(false));
    let mut library = Table::new();
    library.insert("path".to_owned(), path(base, &package.library));
    let dependencies: Table = project
        .dependencies
        .iter()
        .map(|dependency| entry(dependency, base))
        .collect();
    let mut manifest = Table::new();
    manifest.insert("package".to_owned(), Value::from(about));
    manifest.insert("lib".to_owned(), Value::from(library));
    manifest.insert("dependencies".to_owned(), Value::from(dependencies));
    manifest.to_string()
}

/// A dependency as a line of `[dependencies]` names it.
fn entry(dependency: &Dependency, base: &Path) -> (String, Value) {
    let mut entry = Table::new();
    match dependency.source() {
        Source::Path(folder) => entry.insert("path".to_owned(), path(base, folder)),
        Source::Version(requirement) => {
            entry.insert("version".to_owned(), Value::from(requirement.as_str()))
        }
    };
    if !dependency.enabled_features().is_empty() {
        entry.insert(
            "features".to_owned(),
            Value::from(dependency.enabled_features().to_vec()),
        );
    }
    if dependency.default_features().is_off() {
        entry.insert("default-features".to_owned(), Value::from(false));
    }
    (dependency.name().to_owned(), Value::from(entry))
}

fn path(base: &Path, path: &Path) -> Value {
    Value::from(base.join(path).to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests;
