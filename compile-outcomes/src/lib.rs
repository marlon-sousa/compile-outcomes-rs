//! Facade: compiles Rust snippets and reports what the compiler said about each one, without judging.
//!
//! compile-outcomes is made for test suites that check what a library tells its users when they misuse it, such as the errors of a procedural macro. Describe a compilation, compile it, and judge each snippet's outcome as the suite sees fit:
//!
//! ```no_run
//! use compile_outcomes::{Compilation, Dependency, Edition, compile};
//!
//! let compilation = Compilation::new(Edition::E2021, "target/compile-outcomes")
//!     .snippets_in("tests/ui")
//!     .dependency(Dependency::path("greeting", "."));
//!
//! let outcomes = compile(&compilation).unwrap();
//! for (name, outcome) in &outcomes {
//!     for error in outcome.errors() {
//!         println!("{name}: {}", error.message);
//!     }
//! }
//! ```

mod adapters;
mod domain;
mod ports;
#[cfg(test)]
mod test_support;
mod use_cases;

pub use domain::{
    Compilation, Dependency, Diagnostic, Edition, Error, Errors, Location, Outcome, Outcomes,
    SnippetName,
};

use adapters::{CargoProcess, FileSystem};

/// Compiles every snippet of `compilation` and returns the outcome of each, or the errors that kept it from them.
///
/// It writes the project to `work_dir/project`, then runs `cargo build --workspace --keep-going --message-format=json --color=never` there, with `work_dir/target` as the target folder. A snippet that fails to compile is an outcome, never an error.
///
/// It runs the `cargo` the `CARGO` environment variable names, which Cargo sets for the tests it runs, otherwise the one on the `PATH`.
///
/// Two compilations must not share a `work_dir` at the same time, since each replaces the project the other is building. Tests run in parallel by default, so give each test that compiles its own `work_dir`.
///
/// # Errors
///
/// Every problem with the description, such as a snippet whose name is not valid, all together; otherwise the first problem while compiling, such as Cargo failing to start or a dependency failing to compile.
pub fn compile(compilation: &Compilation) -> Result<Outcomes, Errors> {
    use_cases::compile::compile(compilation, &FileSystem, &CargoProcess)
}
