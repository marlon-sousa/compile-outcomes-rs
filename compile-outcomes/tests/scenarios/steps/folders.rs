//! The steps about the folders compile-outcomes replaces or removes, each scenario in a work folder of its own, so that the others keep their build output.

use std::fs;
use std::path::{Path, PathBuf};

use compile_outcomes::{Compilation, compile};
use cucumber::{given, then};

use super::given::edition;
use super::{Compiling, fixture, work_dir};

/// The work folder whose project folder holds a file compile-outcomes did not write.
fn foreign_work_dir() -> PathBuf {
    work_dir().with_extension("foreign")
}

/// The file of its own in the project folder of [`foreign_work_dir`].
fn file_of_its_own() -> PathBuf {
    foreign_work_dir().join("project/notes.txt")
}

/// The work folder of the clean builds.
fn clean_work_dir() -> PathBuf {
    work_dir().with_extension("clean")
}

/// A file left in the build output of [`clean_work_dir`] after a real build, beside what Cargo wrote there.
fn left_behind() -> PathBuf {
    clean_work_dir().join("target/debug/left-behind")
}

/// The file of its own in the target folder of [`clean_work_dir`].
fn target_file_of_its_own() -> PathBuf {
    clean_work_dir().join("target/notes.txt")
}

/// Makes `folder` again, empty, and starts a compilation in it.
fn start_in(world: &mut Compiling, folder: PathBuf, year: u16) {
    if folder.exists() {
        fs::remove_dir_all(&folder).unwrap();
    }
    fs::create_dir_all(&folder).unwrap();
    world.compilation = Some(Compilation::new(edition(year), folder));
}

#[given(expr = "a compilation in edition {int} whose project folder holds a file of its own")]
fn a_compilation_with_a_foreign_project(world: &mut Compiling, year: u16) {
    start_in(world, foreign_work_dir(), year);
    fs::create_dir_all(foreign_work_dir().join("project")).unwrap();
    fs::write(file_of_its_own(), "mine").unwrap();
}

#[given(
    expr = "a compilation in edition {int} whose project folder an earlier compilation wrote, with every file removed and its folders kept"
)]
fn a_compilation_with_a_stripped_project(world: &mut Compiling, year: u16) {
    start_in(world, foreign_work_dir(), year);
    let earlier =
        Compilation::new(edition(year), foreign_work_dir()).snippet(fixture("snippets/unused.rs"));
    compile(&earlier).unwrap();
    remove_files(&foreign_work_dir().join("project"));
}

/// Removes every file under `folder`, keeping the folders, as `Swatinem/rust-cache` does under `target/`.
fn remove_files(folder: &Path) {
    for entry in fs::read_dir(folder).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            remove_files(&path);
        } else {
            fs::remove_file(path).unwrap();
        }
    }
}

#[given("the project folder is handed over to compile-outcomes")]
fn handed_over(_world: &mut Compiling) {
    let marker = foreign_work_dir().join("project/.owned-by-compile-outcomes");
    fs::write(marker, "").unwrap();
}

#[given(
    expr = "a compilation in edition {int} whose build output holds a file an earlier build left"
)]
fn a_compilation_with_a_build_output(world: &mut Compiling, year: u16) {
    start_in(world, clean_work_dir(), year);
    let earlier =
        Compilation::new(edition(year), clean_work_dir()).snippet(fixture("snippets/unused.rs"));
    compile(&earlier).unwrap();
    fs::write(left_behind(), "").unwrap();
}

#[given(expr = "a compilation in edition {int} whose target folder holds a file of its own")]
fn a_compilation_with_a_foreign_target(world: &mut Compiling, year: u16) {
    start_in(world, clean_work_dir(), year);
    fs::create_dir_all(clean_work_dir().join("target")).unwrap();
    fs::write(target_file_of_its_own(), "mine").unwrap();
}

#[given("a clean build")]
fn a_clean_build(world: &mut Compiling) {
    world.describe(Compilation::clean_build);
}

#[then(expr = "the refusal names the project folder and the file {string}")]
fn names_project_folder_and_file(world: &mut Compiling, file: String) {
    names_folder_and_file(world, foreign_work_dir().join("project"), &file);
}

#[then(expr = "the refusal names the target folder and the file {string}")]
fn names_target_folder_and_file(world: &mut Compiling, file: String) {
    names_folder_and_file(world, clean_work_dir().join("target"), &file);
}

fn names_folder_and_file(world: &Compiling, folder: PathBuf, file: &str) {
    let said = world.errors().to_string();
    assert!(said.contains(&folder.display().to_string()), "{said}");
    assert!(said.contains(file), "{said}");
}

#[then("the file of its own is still there")]
fn still_there(_world: &mut Compiling) {
    assert_eq!(fs::read_to_string(file_of_its_own()).unwrap(), "mine");
}

#[then("the file of its own is gone")]
fn gone(_world: &mut Compiling) {
    assert!(!file_of_its_own().exists());
}

#[then("the target folder's file of its own is still there")]
fn target_file_still_there(_world: &mut Compiling) {
    assert_eq!(
        fs::read_to_string(target_file_of_its_own()).unwrap(),
        "mine"
    );
}

#[then("the file the earlier build left is gone")]
fn left_behind_gone(_world: &mut Compiling) {
    assert!(!left_behind().exists());
}

#[then("the file the earlier build left is still there")]
fn left_behind_still_there(_world: &mut Compiling) {
    assert!(left_behind().exists());
}
