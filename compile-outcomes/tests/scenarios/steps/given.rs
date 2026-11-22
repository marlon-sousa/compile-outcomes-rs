//! The steps that describe a compilation.

use std::fs;

use compile_outcomes::{Clippy, Compilation, Dependency, Edition, Group};
use cucumber::gherkin::Step;
use cucumber::given;

use super::{Compiling, fixture, work_dir};

pub(super) fn edition(year: u16) -> Edition {
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
    pub(super) fn describe(&mut self, step: impl FnOnce(Compilation) -> Compilation) {
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

#[given("clippy")]
fn clippy(world: &mut Compiling) {
    world.describe(|compilation| compilation.clippy(Clippy::new()));
}

/// Each row is a level, `allow`, `warn` or `deny`, whether it is for a `group` or a `lint`, and its name.
#[given("clippy with the levels:")]
fn clippy_with_levels(world: &mut Compiling, step: &Step) {
    let table = step.table.as_ref().expect("the step has a table");
    let clippy = table.rows.iter().fold(Clippy::new(), with_level);
    world.describe(|compilation| compilation.clippy(clippy));
}

fn with_level(clippy: Clippy, row: &Vec<String>) -> Clippy {
    match row.as_slice() {
        [level, kind, name] if kind == "group" => match level.as_str() {
            "allow" => clippy.allow(group(name)),
            "warn" => clippy.warn(group(name)),
            "deny" => clippy.deny(group(name)),
            other => panic!("no level {other}"),
        },
        [level, kind, name] if kind == "lint" => match level.as_str() {
            "allow" => clippy.allow_lint(name),
            "warn" => clippy.warn_lint(name),
            "deny" => clippy.deny_lint(name),
            other => panic!("no level {other}"),
        },
        other => panic!("not a level: {other:?}"),
    }
}

fn group(name: &str) -> Group {
    match name {
        "all" => Group::All,
        "correctness" => Group::Correctness,
        "suspicious" => Group::Suspicious,
        "style" => Group::Style,
        "complexity" => Group::Complexity,
        "perf" => Group::Perf,
        "pedantic" => Group::Pedantic,
        "restriction" => Group::Restriction,
        "nursery" => Group::Nursery,
        "cargo" => Group::Cargo,
        other => panic!("no group {other}"),
    }
}
