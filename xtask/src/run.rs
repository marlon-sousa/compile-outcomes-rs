//! Adapter: runs Cargo from the workspace's root, and reads the files the architecture check reads.

mod sources;

use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Command;

pub(crate) use sources::read as sources;

/// The workspace's root, wherever the gate is started from.
pub(crate) fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
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
