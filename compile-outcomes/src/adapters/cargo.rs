//! Adapter: Cargo, run through `std::process`, its messages read with `cargo_metadata`.

mod translation;

use std::env;
use std::ffi::{OsStr, OsString};
use std::io;
use std::path::PathBuf;
use std::process::Command;

use crate::domain::{BuildReport, Error, Project};
use crate::ports::Cargo;

/// The `cargo` the `CARGO` environment variable names, otherwise the one on the `PATH`.
pub(crate) struct CargoProcess;

impl Cargo for CargoProcess {
    fn build(&self, project: &Project) -> Result<BuildReport, Error> {
        let program = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
        let target_dir = env::current_dir()
            .map_err(|error| not_started(&program, &error))?
            .join(&project.target_dir);
        let output = Command::new(&program)
            .args([
                "build",
                "--workspace",
                "--keep-going",
                "--message-format=json",
                "--color=never",
            ])
            .arg("--target-dir")
            .arg(target_dir)
            .current_dir(&project.root)
            .output()
            .map_err(|error| not_started(&program, &error))?;
        let error_output = String::from_utf8_lossy(&output.stderr).into_owned();
        Ok(translation::report(&output.stdout, error_output))
    }
}

fn not_started(program: &OsStr, error: &io::Error) -> Error {
    Error::CargoNotStarted {
        program: PathBuf::from(program),
        reason: error.to_string(),
    }
}
