# Changelog

Every release of compile-outcomes. Each section is written in the release's pull request, from the changelog entries its commits carry as trailers, as [docs/WORKFLOW.md](docs/WORKFLOW.md), "Releases", describes, and becomes the notes of its [GitHub release](https://github.com/marlon-sousa/compile-outcomes-rs/releases). Versions follow [Semantic Versioning](https://semver.org/).

## [0.1.3]

Nothing that worked in 0.1 changes: code written for 0.1.2 compiles unchanged.

### Added

- `Compilation::clippy`, which compiles the snippets with `cargo clippy` instead of `cargo build`, and reports clippy's lints as warnings, or as errors for a denied lint, each with its lint as its code, such as `clippy::let_and_return`. `Clippy` sets the lint levels of clippy's groups, a `Group` such as `Group::Pedantic`, and of single lints by name, in order. Clippy lints only the snippets, never their dependencies.
- `Error::ClippyNotInstalled`, which refuses a compilation with clippy when clippy is not installed for the toolchain, saying how to install it.

## [0.1.2]

### Changed

- compile-outcomes' page on docs.rs shows the README, and the README on crates.io and docs.rs is written for users: it leaves out how to contribute, apart from a pointer to the repository, and its links to the documents and licences lead to them on GitHub.

## [0.1.1]

Nothing a user meets changes: 0.1.1 does what 0.1.0 does, built from tidier code. It is the first version released by the release workflow, publishing to crates.io with trusted publishing.

## [0.1.0]

The first release, the same as 0.1.0-alpha.1, which a snapfit pull request used to check its compile-fail fixtures on Linux, macOS and Windows.

### Added

- `compile`, which compiles Rust snippets and reports, for each one, whether it compiled and every error and warning the compiler gave, as the compiler gave them.
- `compile` replaces the `work_dir/project` folder only when it is missing, holds no file, or holds the file `.owned-by-compile-outcomes`; a folder holding files it did not write is left untouched and the compilation refused. So a folder a CI cache such as rust-cache stripped of its files, keeping its folders, is replaced.
- `Compilation::clean_build`, which removes the build output Cargo made in the work folder before building, through `cargo clean`. It refuses a target folder holding files without `CACHEDIR.TAG`, the file Cargo writes into a target folder it makes, leaving it untouched.

## [0.1.0-alpha.1]

### Added

- `compile`, which compiles Rust snippets and reports, for each one, whether it compiled and every error and warning the compiler gave, as the compiler gave them.
- `compile` replaces the `work_dir/project` folder only when it is missing, holds no file, or holds the file `.owned-by-compile-outcomes`; a folder holding files it did not write is left untouched and the compilation refused. So a folder a CI cache such as rust-cache stripped of its files, keeping its folders, is replaced.
- `Compilation::clean_build`, which removes the build output Cargo made in the work folder before building, through `cargo clean`. It refuses a target folder holding files without `CACHEDIR.TAG`, the file Cargo writes into a target folder it makes, leaving it untouched.
