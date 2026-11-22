// Facade: compiles Rust snippets and reports what the compiler said about each one, without judging.

#![doc = include_str!("../README.md")]

mod adapters;
mod domain;
mod ports;
#[cfg(test)]
mod test_support;
mod use_cases;

pub use domain::{
    Clippy, Compilation, Dependency, Diagnostic, Edition, Error, Errors, Group, Location, Outcome,
    Outcomes, SnippetName,
};

use adapters::{CargoProcess, FileSystem};

/// Compiles every snippet of `compilation` and returns the outcome of each, or the errors that kept it from them.
///
/// It writes the project to `work_dir/project`, then runs `cargo build --workspace --keep-going --message-format=json --color=never` there, with `work_dir/target` as the target folder, or, for a compilation given [`Compilation::clippy`], `cargo clippy` with the same arguments, then `--` and a flag for each of its lint levels, after checking with `cargo clippy --version` that clippy is installed. A snippet that fails to compile is an outcome, never an error.
///
/// It runs the `cargo` the `CARGO` environment variable names, which Cargo sets for the tests it runs, otherwise the one on the `PATH`.
///
/// It replaces `work_dir/project` only when that folder is missing, holds no file at any depth, or holds the file `.owned-by-compile-outcomes`, which it writes there itself; a folder holding other files is left untouched, and the compilation refused.
///
/// Two compilations must not share a `work_dir` at the same time, since each replaces the project the other is building. Tests run in parallel by default, so give each test that compiles its own `work_dir`.
///
/// # Errors
///
/// Every problem with the description, such as a snippet whose name is not valid, all together; otherwise the first problem while compiling, such as Cargo failing to start or a dependency failing to compile.
pub fn compile(compilation: &Compilation) -> Result<Outcomes, Errors> {
    use_cases::compile::compile(compilation, &FileSystem, &CargoProcess)
}
