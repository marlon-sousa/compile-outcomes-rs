//! Policy: the Cargo workspace that compiles a compilation's snippets.

use crate::domain::{Compilation, Edition, Package, Project, Resolver, Snippet};

pub(crate) fn project(compilation: &Compilation, snippets: &[Snippet]) -> Project {
    Project {
        root: compilation.work_dir().join("project"),
        target_dir: compilation.work_dir().join("target"),
        resolver: resolver(compilation.edition()),
        edition: compilation.edition(),
        packages: snippets.iter().map(package).collect(),
        dependencies: compilation.dependencies().to_vec(),
        lockfile: compilation.lockfile_path().map(Into::into),
    }
}

fn package(snippet: &Snippet) -> Package {
    Package {
        name: snippet.name.crate_name(),
        library: snippet.file.clone(),
    }
}

/// The resolver a workspace without a root package must name: the edition's default, and never the first.
fn resolver(edition: Edition) -> Resolver {
    match edition {
        Edition::E2015 | Edition::E2018 | Edition::E2021 => Resolver::V2,
        Edition::E2024 => Resolver::V3,
    }
}

#[cfg(test)]
mod tests;
