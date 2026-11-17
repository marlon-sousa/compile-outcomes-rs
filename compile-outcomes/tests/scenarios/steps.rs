//! The steps of the scenarios, and the compilation they describe.

mod folders;
mod given;
mod then;

use std::fs;
use std::path::PathBuf;

use compile_outcomes::{Compilation, Errors, Outcomes, compile};
use cucumber::{World, when};

/// A compilation being described, and what compiling it gave.
#[derive(Debug, Default, World)]
pub(crate) struct Compiling {
    compilation: Option<Compilation>,
    result: Option<Result<Outcomes, Errors>>,
}

/// The folder every scenario compiles in, one after the other, so that the build of the dependencies is shared.
fn work_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("scenarios")
}

/// Removes the project folder an earlier run left, keeping the build output.
pub(crate) fn forget_earlier_project() {
    let project = work_dir().join("project");
    if project.exists() {
        fs::remove_dir_all(project).unwrap();
    }
}

/// A fixture, as a path relative to the crate's folder, where `cargo test` runs the scenarios.
fn fixture(path: &str) -> PathBuf {
    PathBuf::from("tests/fixtures").join(path)
}

#[when("it is compiled")]
fn it_is_compiled(world: &mut Compiling) {
    let compilation = world
        .compilation
        .as_ref()
        .expect("a compilation was described");
    world.result = Some(compile(compilation));
}
