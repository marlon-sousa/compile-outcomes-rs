# Changelog

Every release of compile-outcomes. Each section is written in the release's pull request, from the changelog entries its commits carry as trailers, as [docs/WORKFLOW.md](docs/WORKFLOW.md), "Releases", describes, and becomes the notes of its [GitHub release](https://github.com/marlon-sousa/compile-outcomes-rs/releases). Versions follow [Semantic Versioning](https://semver.org/).

## [0.1.0-alpha.1]

### Added

- `compile`, which compiles Rust snippets and reports, for each one, whether it compiled and every error and warning the compiler gave, as the compiler gave them.
- `compile` replaces the `work_dir/project` folder only when it is missing, holds no file, or holds the file `.owned-by-compile-outcomes`; a folder holding files it did not write is left untouched and the compilation refused. A folder a CI cache such as rust-cache stripped of its files, keeping its folders, holds no file.
- `Compilation::clean_build`, which removes the build output Cargo made in the work folder before building, through `cargo clean`.
