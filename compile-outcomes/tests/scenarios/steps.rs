//! The steps of the scenarios, and the compilation they describe.

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

/// Removes the project folder a run before the owner marker existed may have left, such as one a CI cache restores, keeping the build output.
pub(crate) fn forget_earlier_projects() {
    let project = work_dir().join("project");
    if project.exists() {
        fs::remove_dir_all(project).unwrap();
    }
}

/// The work folder whose project folder holds a file compile-outcomes did not write.
fn foreign_work_dir() -> PathBuf {
    work_dir().with_extension("foreign")
}

/// The file of its own in the project folder of [`foreign_work_dir`].
fn file_of_its_own() -> PathBuf {
    foreign_work_dir().join("project/notes.txt")
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
