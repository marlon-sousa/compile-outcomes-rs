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
- [ ] Every check of the CI and pull request workflows passes, including the tests on Linux, macOS and Windows and on the minimum supported Rust.
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
- **`.github/workflows/weekly.yml`**, each Monday: the `test` mode on Rust beta, so that a change in the next Rust that breaks compile-outcomes, such as a change to the messages Cargo writes, is known before it is released.

## 8. Versions

compile-outcomes follows [Semantic Versioning](https://semver.org/). Its public API, the names docs/ARCHITECTURE.md, "The public API", lists, is what is promised: a change that breaks code written against it is a major version, or a minor one before 1.0.
