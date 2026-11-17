# Workflow

How a change to compile-outcomes travels from an idea to `main`. [AGENTS.md](../AGENTS.md) holds the rules every change follows and the review checklist; this document holds the process around them.

## 1. The repository

- **Crate:** `compile-outcomes`, used as `compile_outcomes` in code.
- **Repository:** [marlon-sousa/compile-outcomes-rs](https://github.com/marlon-sousa/compile-outcomes-rs).
- **Base branch:** `main`.

## 2. Roles

| Role | Who | What they do |
| :--- | :--- | :--- |
| Maintainer | A person | Decides the public API, the scope of each stack and any trade-off; reviews and merges pull requests. |
| Implementer | An agent or a contributor | Writes the change as [ARCHITECTURE.md](ARCHITECTURE.md) describes, following [AGENTS.md](../AGENTS.md), with the tests it asks for, and runs `cargo xtask ci`. |
| Reviewer | An agent | Started fresh for each change, given the change and not its author's reasoning; works through the checklist in AGENTS.md, "Reviewing a change", and returns findings and a verdict. |

## 3. Branches and commits

- **Branches** are named `stack/<number>-<what it does>`, such as `stack/02-the-model`.
- **Commit messages** follow [Conventional Commits](https://www.conventionalcommits.org/), such as `feat(outcomes): keep the order the compiler gave the diagnostics`.
- **Commit types:** `build`, `chore`, `ci`, `docs`, `feat`, `fix`, `perf`, `refactor`, `revert` or `test`; a formatting-only change is a `refactor`.
- **Changelog entries:** a `feat`, a `fix` or a breaking change (`!` before the colon) ends with at least one trailer, `Added:`, `Changed:`, `Fixed:` or `Removed:`, each a one-line entry of the changelog as a user reads it, or with `Changelog: none` when a user meets no change. For example:

  ```text
  fix(outcomes): report a warning of a snippet that compiled

  Fixed: a snippet that compiles with warnings reports them in its outcome.
  ```

  The pull request workflow checks every commit of a pull request against this convention, with the `commit-messages` action of [itinera-dev/actions](https://github.com/itinera-dev/actions). Merge commits are not checked.

## 4. Stacked pull requests

Each pull request does one thing and stays small, ideally under 300 lines of code. A change too big for one is split into a stack: each layer is a branch on the layer below, and its pull request targets that branch, so each can be read alone. When a layer merges, the next one is rebased onto `main` and retargeted.

## 5. The review loop

1. **Implementation:** the implementer writes the change and its tests, and runs `cargo xtask ci`.
2. **Independent review:** before the pull request is opened, a reviewer agent reviews the commits against the checklist in AGENTS.md, "Reviewing a change"; `.claude/skills/review-change` says how Claude starts it. Findings are fixed, or rejected with a reason the reviewer accepts or the maintainer decides. Every later push is reviewed the same way.
3. **CI:** GitHub Actions runs the checks of section 7.
4. **Sign-off:** the pull request is reported ready, with the review's verdict, once both the review and the CI of its latest push have passed. The maintainer approves and merges, or advances the stack.

## 6. Definition of done

A pull request is ready to merge when:

- [ ] `cargo xtask ci` passes locally.
- [ ] Every behaviour has its scenario, and every fixed bug its regression test, as AGENTS.md, "Rules", asks.
- [ ] Every check of the CI and pull request workflows passes, and of the release workflow when the change alters how releases are made, including the tests on Linux, macOS and Windows and on the minimum supported Rust.
- [ ] The description has no unticked checkbox.
- [ ] The reviewer agent's verdict on the latest push is approve.

## 7. Checks

`cargo xtask ci` runs every check below, in this order. Each runs on its own as `cargo xtask <mode>`:

| Mode | What it checks |
| :--- | :--- |
| `fmt` | rustfmt compliance. |
| `clippy` | clippy with every warning an error. |
| `arch` | no input or output in `domain/` or `use_cases/`, no `mod.rs`, and files of at most 200 lines. |
| `test` | every unit test and Gherkin scenario. |
| `doc` | the documentation, with every warning an error. |
| `deny` | the licences, advisories and sources of the dependencies, as `deny.toml` allows them. It needs cargo-deny, installed with `cargo install cargo-deny --version 0.20.2 --locked`. |
| `package` | the crate built from its package alone, as crates.io would receive it, with `cargo publish --workspace --dry-run`, in its own target folder. |

### The toolchain

`rust-toolchain.toml` pins the toolchain every contributor, cloud session and CI job builds with, a recent stable Rust. The minimum Rust version promised to users is `rust-version` in `Cargo.toml`, 1.85.0, which its own CI job checks. Raising either is a change of its own.

### GitHub Actions

Every action a workflow uses is pinned to a commit, with its version in a comment beside it.

- **`.github/workflows/ci.yml`**, on every pull request and every push to `main` or a `stack/*` branch: each mode of `cargo xtask ci` in a job of its own on Linux, the `test` mode on Linux, macOS and Windows, and the `test` mode again with the minimum supported Rust.
- **`.github/workflows/pull-request.yml`**, on every pull request, whatever its base: every commit message keeps the convention of section 3, and the description has no unticked checkbox. Ticking a checkbox runs only this workflow again.
- **`.github/workflows/release.yml`**, on a tag `v<version>`, and on a pull request that changes how releases are made: section 9 describes it.
- **`.github/workflows/weekly.yml`**, each Monday: the `test` mode on Rust beta, so that a change in the next Rust that breaks compile-outcomes, such as a change to the messages Cargo writes, is known before it is released.

## 8. Versions

compile-outcomes follows [Semantic Versioning](https://semver.org/). Its public API, the names docs/ARCHITECTURE.md, "The public API", lists, is what is promised: a change that breaks code written against it is a major version, or a minor one before 1.0.

## 9. Releases

compile-outcomes has one version, `version` in the workspace's `Cargo.toml`.

### Preparing a release

A release starts as a pull request, titled "Release <version>", whose commit `chore(release): release <version>`:

1. raises `version` under `[workspace.package]` in `Cargo.toml`, and the version in `Cargo.lock` with `cargo update --workspace`;
2. writes the version's section at the top of [CHANGELOG.md](../CHANGELOG.md), starting from what `git cliff --unreleased --tag v<version> --strip all` prints, which is the heading `## [<version>]` and the entries of the commits since the last release;
3. reconciles those entries, since a commit's entries cannot change once it merges: entries a later commit made untrue become one that says what the release holds, entries that say the same thing merge, and a change a user meets that carries no entry gets one.

`cliff.toml` is written for git-cliff 2.14.2, which installs with `cargo install git-cliff --version 2.14.2 --locked`.

A pre-release is named with a dot before its number, such as `0.1.0-alpha.1` or `0.2.0-rc.1`. Only a version without a pre-release part and a release candidate, `rc.N`, go to crates.io; an alpha, a beta or any other pre-release is released on GitHub only.

### Tagging it

Once the pull request has merged, the maintainer tags the commit the merge put on `main` and pushes the tag, such as `git tag -a v0.1.0 <commit> -m "compile-outcomes 0.1.0"` and `git push origin v0.1.0`. `.github/workflows/release.yml` then runs:

1. **release gate**, which refuses the tag unless it is `v` and the version in `Cargo.toml`, its commit is on `main`, and `CHANGELOG.md` has notes for the version, reporting every reason, and tells whether the version goes to crates.io;
2. **checks**, every job of `ci.yml` again on the tagged commit;
3. **release tools**, the release gate and the crates.io rule against their fixtures under `.github/release/`;
4. **the crate's package**, the `.crate` file of compile-outcomes;
5. **crates.io**, for a version that goes there, once every job before it passed;
6. **GitHub release**, with the `create-release` action of [itinera-dev/actions](https://github.com/itinera-dev/actions): the version's section of `CHANGELOG.md` as its notes, a pre-release for a version such as `0.2.0-rc.1`, and the package attached.

A pull request that changes how releases are made runs the release tools and packs the crate, without releasing anything.

### Publishing to crates.io

crates.io's trusted publishing lets the workflow publish without a token, but it is set up on a crate that already exists there. So the first version, 0.1.0, is published by hand: once the checks on the tagged commit and the crates.io dry run have passed, the maintainer runs `cargo publish --package compile-outcomes --locked` at the tagged commit, from Linux or macOS. The crate's licence files are links to the ones at the root, which a Windows checkout without symbolic links would pack as text. Until trusted publishing is set up, the crates.io job only runs `cargo publish --workspace --dry-run --locked`. A later pull request then makes the job publish, in a `crates-io` environment, with the `publish-crates` action of itinera-dev/actions.
