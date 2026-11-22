//! Adapter: Cargo, run through `std::process` to check for clippy, to clean and to build, its messages read with `cargo_metadata`.

mod clippy_flags;
mod translation;

use std::env;
use std::ffi::{OsStr, OsString};
use std::io;
use std::path::PathBuf;
use std::process::{Command, Output};

use crate::domain::{BuildReport, Error, Project, Tool};
use crate::ports::Cargo;

/// What every build passes Cargo, whatever the tool.
const BUILD_ARGS: [&str; 4] = [
    "--workspace",
    "--keep-going",
    "--message-format=json",
    "--color=never",
];

/// The `cargo` the `CARGO` environment variable names, otherwise the one on the `PATH`.
pub(crate) struct CargoProcess;

impl Cargo for CargoProcess {
    fn clippy_installed(&self) -> Result<(), Error> {
        let program = program();
        let output = Command::new(&program)
            .args(["clippy", "--version"])
            .output()
            .map_err(|error| not_started(&program, &error))?;
        if output.status.success() {
            Ok(())
        } else {
            Err(Error::ClippyNotInstalled {
                program: PathBuf::from(program),
                output: String::from_utf8_lossy(&output.stderr).into_owned(),
            })
        }
    }

    fn clean(&self, project: &Project) -> Result<(), Error> {
        let output = run(project, "clean", &["--color=never".into()])?;
        if output.status.success() {
            Ok(())
        } else {
            Err(Error::TargetFolderNotCleaned {
                folder: project.target_dir.clone(),
                reason: String::from_utf8_lossy(&output.stderr).into_owned(),
            })
        }
    }

    fn build(&self, project: &Project, tool: &Tool) -> Result<BuildReport, Error> {
        let args = BUILD_ARGS.map(String::from);
        let output = match tool {
            Tool::Build => run(project, "build", &args)?,
            Tool::Clippy(clippy) => {
                let flags = clippy_flags::flags(clippy);
                run(project, "clippy", &[&args[..], &flags].concat())?
            }
        };
        let error_output = String::from_utf8_lossy(&output.stderr).into_owned();
        Ok(translation::report(&output.stdout, error_output))
    }
}

fn program() -> OsString {
    env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"))
}

/// Runs the Cargo `command` with `args` in the project's root, with the project's target folder made absolute.
fn run(project: &Project, command: &str, args: &[String]) -> Result<Output, Error> {
    let program = program();
    let target_dir = env::current_dir()
        .map_err(|error| not_started(&program, &error))?
        .join(&project.target_dir);
    Command::new(&program)
        .arg(command)
        .arg("--target-dir")
        .arg(target_dir)
        .args(args)
        .current_dir(&project.root)
        .output()
        .map_err(|error| not_started(&program, &error))
}

fn not_started(program: &OsStr, error: &io::Error) -> Error {
    Error::CargoNotStarted {
        program: PathBuf::from(program),
        reason: error.to_string(),
    }
}
