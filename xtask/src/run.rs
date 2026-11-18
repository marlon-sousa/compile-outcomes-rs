//! Adapter: runs Cargo from the workspace's root, reads the files the architecture and README checks read, and writes the crate's README.

mod sources;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub(crate) use sources::read as sources;

/// The workspace's root, wherever the gate is started from.
pub(crate) fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// The text of the file at `path`, from the workspace's root, or the reason the system gave.
pub(crate) fn text(path: &Path) -> Result<String, String> {
    fs::read_to_string(root().join(path)).map_err(|error| error.to_string())
}

/// Writes `text` to the file at `path`, from the workspace's root, or gives a sentence with the reason the system gave.
pub(crate) fn write(path: &Path, text: &str) -> Result<(), String> {
    fs::write(root().join(path), text)
        .map_err(|error| format!("{} could not be written: {error}", path.display()))
}

/// Runs Cargo with `arguments`, and tells whether it succeeded.
pub(crate) fn cargo(arguments: &[&str]) -> bool {
    succeeds(&mut command(arguments))
}

/// Runs Cargo with `arguments` and every warning of rustdoc an error, and tells whether it succeeded.
pub(crate) fn rustdoc(arguments: &[&str]) -> bool {
    succeeds(command(arguments).env("RUSTDOCFLAGS", "-D warnings"))
}

fn command(arguments: &[&str]) -> Command {
    println!("\n>>> cargo {}", arguments.join(" "));
    let program = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
    let mut command = Command::new(program);
    command.args(arguments).current_dir(root());
    command
}

fn succeeds(command: &mut Command) -> bool {
    match command.status() {
        Ok(status) => status.success(),
        Err(error) => {
            eprintln!("Cargo could not be started: {error}");
            false
        }
    }
}
