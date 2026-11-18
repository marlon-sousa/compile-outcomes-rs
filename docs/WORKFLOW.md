# Workflow

How a change to compile-outcomes travels from an issue to `main`. [AGENTS.md](../AGENTS.md) holds the rules every change follows and the review checklist; this document holds the process around them.

## 1. The repository

- **Crate:** `compile-outcomes`, used as `compile_outcomes` in code.
- **Repository:** [marlon-sousa/compile-outcomes-rs](https://github.com/marlon-sousa/compile-outcomes-rs).
- **Base branch:** `main`.

## 2. Roles

| Role | Who | What they do |
| :--- | :--- | :--- |
| Maintainer | A person | Agrees the issues, decides the public API, the scope of each stack and any trade-off; reviews and merges pull requests. |
| Implementer | An agent or a contributor | Writes the change as [ARCHITECTURE.md](ARCHITECTURE.md) describes, following [AGENTS.md](../AGENTS.md), with the tests it asks for, and runs `cargo xtask ci`. |
| Reviewer | An agent | Started fresh for each change, given the change and not its author's reasoning; works through the checklist in AGENTS.md, "Reviewing a change", and returns findings and a verdict. |

## 3. Branches and commits

- **Branches** are named freely, after what they do; a stack is the chain of its pull requests, as section 4 says, not of its branch names.
- **Commit messages** follow [Conventional Commits](https://www.conventionalcommits.org/), such as `feat(outcomes): keep the order the compiler gave the diagnostics`.
- **Commit types:** `build`, `chore`, `ci`, `docs`, `feat`, `fix`, `perf`, `refactor`, `revert` or `test`; a formatting-only change is a `refactor`.
- **Changelog entries:** a `feat`, a `fix` or a breaking change (`!` before the colon) ends with at least one trailer, `Added:`, `Changed:`, `Fixed:` or `Removed:`, each a one-line entry of the changelog as a user reads it, or with `Changelog: none` when a user meets no change. For example:

  ```text
  fix(outcomes): report a warning of a snippet that compiled

  Fixed: a snippet that compiles with warnings reports them in its outcome.
  ```

  The pull request workflow checks every commit of a pull request against this convention, with the `commit-messages` action of [itinera-dev/actions](https://github.com/itinera-dev/actions). Merge commits are not checked.

### Naming the issue

Every pull request names an open issue of this repository, as section 10 says, unless every one of its commits, merges aside, is a `test`: `Closes #N` when it finishes the issue, `Refs #N` when it does part of it. The layers of a stack each say `Refs #N`, and the top one also says `Closes #N` once it targets `main`: GitHub links a closing keyword only on a pull request into the default branch, so before that the check finds only `Refs #N`. A pull request doing the work of several issues names each of them. A pull request never closes an issue of another repository, which GitHub would close the moment it merged, before that repository's own work.

## 4. Stacked pull requests

Each pull request does one thing and stays small, ideally under 300 lines of code. A change too big for one is split into a stack: each layer is a branch on the layer below, and its pull request targets that branch, so each can be read alone. When a layer merges, the next one is rebased onto `main` and retargeted.

## 5. The review loop

1. **Implementation:** starting from its agreed issue, the implementer writes the change and its tests, and runs `cargo xtask ci`.
2. **Independent review:** before the pull request is opened, a reviewer agent reviews the commits, given the issue they belong to, against the checklist in AGENTS.md, "Reviewing a change"; `.claude/skills/review-change` says how Claude starts it. Findings are fixed, or rejected with a reason the reviewer accepts or the maintainer decides. Every later push is reviewed the same way.
3. **CI:** GitHub Actions runs the checks of section 7.
4. **Sign-off:** the pull request is reported ready, with the review's verdict, once both the review and the CI of its latest push have passed. The maintainer approves and merges, or advances the stack.

## 6. Definition of done

A pull request is ready to merge when:

- [ ] The description names its open issues, with `Closes #N` or `Refs #N`, unless every commit, merges aside, is a `test`.
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
| `readme` | each code block of README.md marked `<!-- example: <file> -->` is held, as written, by the file it names, which should belong to a crate the gate builds, such as `examples/my_macros`, whose tests run in the `test` mode. |
| `test` | every unit test and Gherkin scenario, the README's examples included. |
| `doc` | the documentation, with every warning an error. |
| `deny` | the licences, advisories and sources of the dependencies, as `deny.toml` allows them. It needs cargo-deny, installed with `cargo install cargo-deny --version 0.20.2 --locked`. |
| `package` | the crate built from its package alone, as crates.io would receive it, with `cargo publish --workspace --dry-run`, in its own target folder. |

### The toolchain

`rust-toolchain.toml` pins the toolchain every contributor, cloud session and CI job builds with, a recent stable Rust. The minimum Rust version promised to users is `rust-version` in `Cargo.toml`, 1.85.0, which its own CI job checks. Raising either is a change of its own.

### GitHub Actions

Every action a workflow uses is pinned to a commit, with its version in a comment beside it.

- **`.github/workflows/ci.yml`**, on every pull request, the layers of a stack included, and every push to `main`: each mode of `cargo xtask ci` in a job of its own on Linux, the `test` mode on Linux, macOS and Windows, and the `test` mode again with the minimum supported Rust.
- **`.github/workflows/pull-request.yml`**, on every pull request, whatever its base: the description names an open issue of this repository, unless every commit other than a merge is a `test`, and closes none of another, with the `pr-has-issue` and `no-cross-repo-closing` actions of itinera-dev/actions; every commit message keeps the convention of section 3; and the description has no unticked checkbox. Ticking a checkbox runs only this workflow again.
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
5. **crates.io**, for a version that goes there, once every job before it passed, publishing it as the next part says;
6. **GitHub release**, with the `create-release` action of [itinera-dev/actions](https://github.com/itinera-dev/actions): the version's section of `CHANGELOG.md` as its notes, a pre-release for a version such as `0.2.0-rc.1`, and the package attached.

A pull request that changes how releases are made runs the release tools and packs the crate, without releasing anything.

### Publishing to crates.io

The crates.io job publishes with crates.io's trusted publishing, so the repository keeps no token. crates.io trusts `release.yml` of this repository in the GitHub environment `crates-io`, which only tags `v*` may deploy to. The job asks GitHub for a token proving where it runs, `rust-lang/crates-io-auth-action` exchanges it for a short-lived crates.io token, and `cargo publish --package compile-outcomes --locked` uses it. It runs on Linux, since the crate's licence files are links to the ones at the root, which a Windows checkout without symbolic links would pack as text.

Trusted publishing is set up on a crate that already exists on crates.io, so the first version, 0.1.0, was published by hand by the maintainer.

## 10. Issues come first

Every change starts as an issue, and is agreed there before any code is written: building the wrong thing is the expensive mistake, and the issue is the cheapest place to find it. The issues are also the record of every decision taken about compile-outcomes, what it does and how it is built and released, so that a decision can be found again by its kind.

The one exception is a change whose commits, merges aside, are all `test` commits. It changes neither what compile-outcomes does nor how its code is organised or released, so it holds no decision of its own: what it checks was agreed already. It may still add a dependency used only by tests, which the review checks as AGENTS.md, rule 9, asks.

- **A feature** says what is wanted and why, and what it must not break. It can be short; the details are worked out in the issue.
- **A bug** says what happened, what was expected, and the smallest snippet or compilation that shows it.
- **A change to the architecture, the process or the documentation** says what changes and why, and what else was considered.
- **Labels:** each issue carries one label, which says what kind of decision it holds, so that decisions can be found again:
  - `feature`, something new a user meets;
  - `bug`, something a user meets that is wrong;
  - `architecture`, how the code is organised or written, refactors included, and the rules on code of docs/ARCHITECTURE.md and AGENTS.md;
  - `process`, CI, this workflow, releases, and the process rules and review checklist of AGENTS.md;
  - `docs`, the documentation a user reads, such as README.md.

  One issue per kind: a pull request that holds decisions of several kinds names one issue of each kind.

A decision the maintainer takes about an issue, wherever it is taken, such as in a chat, is recorded at once as a comment on the issue, headed "Decided so far (continued)", dated, saying that the maintainer decided it, and saying so when an agent wrote it. Once an issue is agreed, every decision taken in its comments is written into its body, which then binds the work. The comments stay as the history of how each decision was reached, and nothing in them may contradict the body.

docs/SPEC.md describes what ships; the open `feature` issues hold what is planned. A feature is written into docs/SPEC.md in the pull request that builds it, as AGENTS.md, rule 1, asks.
