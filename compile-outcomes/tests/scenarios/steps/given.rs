//! The steps that describe a compilation.

use std::fs;

use compile_outcomes::{Compilation, Dependency, Edition};
use cucumber::given;

use super::{Compiling, file_of_its_own, fixture, foreign_work_dir, work_dir};

fn edition(year: u16) -> Edition {
    match year {
        2015 => Edition::E2015,
        2018 => Edition::E2018,
        2021 => Edition::E2021,
        2024 => Edition::E2024,
        other => panic!("no edition {other}"),
    }
}

fn greeting() -> Dependency {
    Dependency::path("greeting", fixture("greeting"))
}

impl Compiling {
    fn describe(&mut self, step: impl FnOnce(Compilation) -> Compilation) {
        let compilation = self.compilation.take().expect("a compilation was started");
        self.compilation = Some(step(compilation));
    }
}

#[given(expr = "a compilation in edition {int}")]
fn a_compilation(world: &mut Compiling, year: u16) {
    world.compilation = Some(Compilation::new(edition(year), work_dir()));
}

#[given(expr = "a compilation in edition {int} whose work folder is a file")]
fn a_compilation_in_a_file(world: &mut Compiling, year: u16) {
    let file = work_dir().with_extension("file");
    fs::write(&file, "").unwrap();
    world.compilation = Some(Compilation::new(edition(year), file));
}

#[given(expr = "a compilation in edition {int} whose project folder holds a file of its own")]
fn a_compilation_with_a_foreign_project(world: &mut Compiling, year: u16) {
    let folder = foreign_work_dir();
    if folder.exists() {
        fs::remove_dir_all(&folder).unwrap();
    }
    fs::create_dir_all(folder.join("project")).unwrap();
    fs::write(file_of_its_own(), "mine").unwrap();
    world.compilation = Some(Compilation::new(edition(year), folder));
}

#[given("the project folder is handed over to compile-outcomes")]
fn handed_over(_world: &mut Compiling) {
    let marker = foreign_work_dir().join("project/.owned-by-compile-outcomes");
    fs::write(marker, "").unwrap();
}

#[given(expr = "the snippet {string}")]
fn the_snippet(world: &mut Compiling, path: String) {
    world.describe(|compilation| compilation.snippet(fixture(&path)));
}

#[given(expr = "the snippets in {string}")]
fn the_snippets_in(world: &mut Compiling, folder: String) {
    world.describe(|compilation| compilation.snippets_in(fixture(&folder)));
}

#[given("the dependency greeting")]
fn the_dependency_greeting(world: &mut Compiling) {
    world.describe(|compilation| compilation.dependency(greeting()));
}

#[given(expr = "the dependency greeting with the feature {string}")]
fn greeting_with_a_feature(world: &mut Compiling, feature: String) {
    world.describe(|compilation| compilation.dependency(greeting().features([feature])));
}

#[given("the dependency greeting without its default features")]
fn greeting_without_default_features(world: &mut Compiling) {
    world.describe(|compilation| compilation.dependency(greeting().without_default_features()));
}

#[given(expr = "the dependency {string} at {string}")]
fn a_dependency_at(world: &mut Compiling, name: String, path: String) {
    world.describe(|compilation| compilation.dependency(Dependency::path(name, path)));
}

#[given(expr = "the dependency {string} of version {string}")]
fn a_dependency_of_version(world: &mut Compiling, name: String, version: String) {
    world.describe(|compilation| compilation.dependency(Dependency::version(name, version)));
}

#[given(expr = "the lockfile {string}")]
fn the_lockfile(world: &mut Compiling, path: String) {
    world.describe(|compilation| compilation.lockfile(fixture(&path)));
}
