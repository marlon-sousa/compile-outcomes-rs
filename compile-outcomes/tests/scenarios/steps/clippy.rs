//! The steps that ask for clippy, with its lint levels and its configuration.

use std::fs;
use std::path::PathBuf;

use compile_outcomes::{Clippy, Compilation, Group};
use cucumber::gherkin::Step;
use cucumber::given;

use super::given::edition;
use super::{Compiling, work_dir};

/// The folder holding a `clippy.toml` and, below it, the work folder of [`a_compilation_below_a_clippy_toml`].
fn configured() -> PathBuf {
    work_dir().with_extension("configured")
}

#[given(
    expr = "a compilation in edition {int} whose work folder is below a clippy.toml setting too-many-arguments-threshold to {int}"
)]
fn a_compilation_below_a_clippy_toml(world: &mut Compiling, year: u16, threshold: u32) {
    fs::create_dir_all(configured()).unwrap();
    let setting = format!("too-many-arguments-threshold = {threshold}\n");
    fs::write(configured().join("clippy.toml"), setting).unwrap();
    world.compilation = Some(Compilation::new(edition(year), configured().join("work")));
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
