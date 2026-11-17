use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use tempfile::TempDir;

use super::FileSystem;
use crate::domain::{Edition, Error, OWNER_MARKER, Package, Project, Resolver};
use crate::ports::Disk;

fn project(root: &Path, lockfile: Option<PathBuf>) -> Project {
    Project {
        root: root.join("project"),
        target_dir: root.join("target"),
        resolver: Resolver::V2,
        edition: Edition::E2021,
        packages: vec![Package {
            name: "snippet_a".to_owned(),
            library: root.join("a.rs"),
        }],
        dependencies: Vec::new(),
        lockfile,
    }
}

#[test]
fn a_folder_lists_its_files_in_order_without_its_subfolders() {
    let folder = TempDir::new().unwrap();
    fs::write(folder.path().join("b.rs"), "").unwrap();
    fs::write(folder.path().join("a.rs"), "").unwrap();
    fs::create_dir(folder.path().join("nested.rs")).unwrap();

    assert_eq!(
        FileSystem.files_in(folder.path()),
        Ok(BTreeSet::from([
            folder.path().join("a.rs"),
            folder.path().join("b.rs")
        ]))
    );
}

#[test]
fn a_file_that_is_there_can_be_read() {
    assert_eq!(FileSystem.readable(Path::new("Cargo.toml")), Ok(()));
}

#[test]
fn a_missing_file_or_folder_gives_the_reason_the_system_gave() {
    let folder = TempDir::new().unwrap();
    let missing = folder.path().join("missing");
    let unopened = fs::metadata(&missing).unwrap_err().to_string();
    let unlisted = fs::read_dir(&missing).unwrap_err().to_string();

    assert_eq!(FileSystem.readable(&missing), Err(unopened));
    assert_eq!(FileSystem.files_in(&missing), Err(unlisted));
}

#[test]
fn a_folder_is_not_a_file_that_can_be_read() {
    let folder = TempDir::new().unwrap();
    let reason = io::Error::from(io::ErrorKind::IsADirectory).to_string();

    assert_eq!(FileSystem.readable(folder.path()), Err(reason));
}

#[test]
fn the_project_replaces_what_an_earlier_one_left() {
    let folder = TempDir::new().unwrap();
    let stale = folder.path().join("project/snippet_old");
    fs::create_dir_all(&stale).unwrap();

    FileSystem.write(&project(folder.path(), None)).unwrap();

    assert!(!stale.exists());
    let root = fs::read_to_string(folder.path().join("project/Cargo.toml")).unwrap();
    assert!(root.contains("snippet_a"), "{root}");
    assert!(folder.path().join("project/snippet_a/Cargo.toml").is_file());
    assert!(folder.path().join("project").join(OWNER_MARKER).is_file());
    assert!(!folder.path().join("project/Cargo.lock").exists());
}

#[test]
fn the_lockfile_is_copied_and_left_as_it_was() {
    let folder = TempDir::new().unwrap();
    let lockfile = folder.path().join("Cargo.lock");
    fs::write(&lockfile, "version = 4\n").unwrap();

    FileSystem
        .write(&project(folder.path(), Some(lockfile.clone())))
        .unwrap();

    let copy = fs::read_to_string(folder.path().join("project/Cargo.lock")).unwrap();
    assert_eq!(copy, "version = 4\n");
    assert_eq!(fs::read_to_string(&lockfile).unwrap(), "version = 4\n");
}

#[test]
fn a_lockfile_that_cannot_be_copied_names_the_copy() {
    let folder = TempDir::new().unwrap();
    let lockfile = folder.path().join("missing.lock");

    let error = FileSystem
        .write(&project(folder.path(), Some(lockfile.clone())))
        .unwrap_err();

    let copy = folder.path().join("project/Cargo.lock");
    assert!(matches!(error, Error::Unwritable { path, .. } if path == copy));
}

#[test]
fn a_missing_folder_has_no_names() {
    let folder = TempDir::new().unwrap();

    assert_eq!(
        FileSystem.names_in(&folder.path().join("missing")),
        Ok(None)
    );
}

#[test]
fn the_names_in_a_folder_are_those_of_its_files_and_folders() {
    let folder = TempDir::new().unwrap();
    fs::write(folder.path().join("notes.txt"), "").unwrap();
    fs::create_dir(folder.path().join("drafts")).unwrap();

    let names = FileSystem.names_in(folder.path());

    let expected = BTreeSet::from([OsString::from("drafts"), OsString::from("notes.txt")]);
    assert_eq!(names, Ok(Some(expected)));
}

#[test]
fn removing_a_folder_removes_everything_in_it_and_a_missing_one_is_already_removed() {
    let folder = TempDir::new().unwrap();
    let target = folder.path().join("target");
    fs::create_dir_all(target.join("debug")).unwrap();
    fs::write(target.join("debug/stale"), "").unwrap();

    assert_eq!(FileSystem.remove(&target), Ok(()));
    assert!(!target.exists());
    assert_eq!(FileSystem.remove(&target), Ok(()));
}

#[test]
fn a_file_that_cannot_be_removed_as_a_folder_names_it() {
    let folder = TempDir::new().unwrap();
    let file = folder.path().join("target");
    fs::write(&file, "").unwrap();

    let error = FileSystem.remove(&file).unwrap_err();

    assert!(matches!(error, Error::Unremovable { folder, .. } if folder == file));
}
