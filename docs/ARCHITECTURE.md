# Architecture

How compile-outcomes' code is organised and written. [SPEC.md](SPEC.md) says what it does; this document says how the code that does it is laid out, and the rules every change follows. [AGENTS.md](../AGENTS.md) says how a change is made and reviewed against it. Changing a rule here is agreed with the maintainer first, like any other change.

The rules are those of [snapfit](https://github.com/marlon-sousa/snapfit-rs/blob/main/docs/ARCHITECTURE.md), adapted to a library that writes files and runs a program, where snapfit only reads and writes tokens.

## The guiding principle

The domain decides; the adapters translate. The pure part of compile-outcomes, which decides what the snippets are called, what project compiles them and what each one's outcome is, does no input or output. The adapters at the edges write the project to disk, run Cargo and read its messages, and decide nothing about snippets or outcomes.

compile-outcomes does input and output, so it has ports: a port is a trait the service needs the world to implement, so that the service's tests can give it a fake world. A pure function never gets a trait around it: it is already testable.

## The repository

- **`compile-outcomes/`**, the crate users depend on, laid out as the rest of this document describes. Its `tests/` folder holds the Gherkin scenarios: the features, `tests/features/`; their steps, the test target `tests/scenarios/`; and what they compile, `tests/fixtures/`, snippets and two small crates, `greeting` and `broken`, which the workspace excludes.
- **`examples/my_macros/`**, a crate that is never published, holding the README's examples as files: a `#[test]`, a Gherkin feature and its steps, and the snippets they compile. The `test` mode runs them, and the `readme` mode checks that the README shows them as written.
- **`xtask/`**, the quality gate, `cargo xtask ci`, whose checks CI runs, the tests on Linux, macOS and Windows; [WORKFLOW.md](WORKFLOW.md), section 7, lists them.
- **`.github/workflows/`**, the CI, pull request, release and weekly workflows, as [WORKFLOW.md](WORKFLOW.md), section 7, lists them, and **`.github/release/`**, the scripts of the release workflow and their fixtures.
- **`cliff.toml`**, how git-cliff drafts a release's section of **`CHANGELOG.md`**, as [WORKFLOW.md](WORKFLOW.md), section 9, says.
- **`clippy.toml`**, what clippy allows in tests.
- **`deny.toml`**, what cargo-deny allows of the dependencies.
- **`docs/`**: [SPEC.md](SPEC.md), what compile-outcomes does; this document; [WORKFLOW.md](WORKFLOW.md), the stacked pull requests, the definition of done and how a version is released.
- **`.claude/skills/`**, the procedures an agent follows, such as having a change reviewed. They add no rule of their own: each points at the rules it follows.

## How a compilation runs

1. `lib.rs` re-exports the model and defines `compile`, which hands the compilation to the service with the real adapters.
2. The service, `use_cases/compile.rs`, asks the disk port for the snippet files the compilation names, and the naming policy for the snippets they make. It gathers every problem with the description before going on.
3. The project policy makes the `Project` that compiles those snippets: one package per snippet, with the compilation's edition and dependencies.
4. The disk port lists the names at the project's root, the ownership policy says whose that folder is, and the service refuses a folder holding files compile-outcomes did not write, and for a clean build a target folder Cargo did not make. Otherwise the disk port writes the project, the Cargo port cleans the build output for a clean build, and the Cargo port builds it and returns the `BuildReport`: every compiler message with the crate it is about, every crate built, and what Cargo wrote to its error output.
5. The outcomes policy turns the report into the `Outcomes`, or into the errors that kept the compilation from them.

## Module roles

Every module plays one role, and says which on the first line of its `//!` doc comment, such as `//! Policy: the outcome of every snippet, from what Cargo reported.`

1. **Entity**: data and the invariants it keeps, in `domain/`: the compilation, the snippets and their names, the project, the build report and the outcomes.
2. **Error**: what keeps a compilation from its outcomes, in `domain/errors.rs`, as "Errors" below describes.
3. **Policy**: a pure decision, in `use_cases/`: naming the snippets, making the project, whose the project and target folders are, and the outcomes.
4. **Service**: `use_cases/compile.rs`, which runs a compilation through the ports and the policies.
5. **Port**: a trait the service needs the world to implement, in `ports.rs`: the disk and Cargo.
6. **Adapter**: the code that does input and output, in `adapters/`: the disk adapter, which speaks `std::fs` and `toml`, and the Cargo adapter, which speaks `std::process` and `cargo_metadata`. Pure code that only one adapter uses to speak its library, such as turning a compiler message into a `Diagnostic`, is a private submodule of that adapter.
7. **Facade**: `lib.rs`, which re-exports the model and holds `compile`, a one-line call to the service.

A submodule that serves a single module plays its parent's role and says so on its role line.

`xtask/` is a tool, not part of the library, and uses three roles: `main.rs` is its **Handler**, which reads the command line and reports the verdict; the modes, the architecture rules and the README's examples are **Policies**; running Cargo and reading the source files, the README and the files it shows are its **Adapters**. The handler calls both, and neither calls the other.

Layering goes one way. The facade calls the service with the adapters; the service calls ports and policies; policies use entities and errors; adapters implement ports and use entities and errors, never a policy or the service. No module reaches around another into its internals.

## The typed model

- **The domain does no input or output.** `domain/` and `use_cases/` name nothing of `std::fs`, `std::io`, `std::process`, `std::env`, `std::net`, `std::os`, `cargo_metadata` or `toml`, call no method that opens a path, such as `exists`, `metadata` or `read_dir`, whatever it is called on, and print nothing with `print!`, `println!`, `eprint!`, `eprintln!` or `dbg!`. `cargo xtask ci` checks all three, reading each file's syntax tree and the arguments of its macros, and refuses importing `std` whole, which would hide what it uses. Paths are data there, never opened.
- **Adapters translate, they don't decide.** The Cargo adapter reports every message with the crate it is about and its level, and every crate built; which crate is a snippet, whether a snippet compiled and whether a dependency failed are the outcomes policy's. The disk adapter writes the project the project policy made, and lists the files of a folder, or the names of its entries that hold a file; which files are snippets and what they are called is the naming policy's, and whether the project folder is compile-outcomes' own, or the target folder Cargo's, is the ownership policy's.
- **A snippet's name is checked once.** `SnippetName` can only be made from a valid name, so every later step holds a name that is known to be good, and the crate name the compiler reports maps back to its snippet without a lookup that could fail.

## Folders and files

1. **The visibility ladder.** Private by default, `pub(crate)` when another module needs it, and bare `pub` only for the public API that `lib.rs` re-exports. The `unreachable_pub` lint, set in the workspace's `[workspace.lints]`, catches a `pub` that is not re-exported.
2. **`module.rs` with a `module/` folder, never `mod.rs`**, so that every file's name says what it holds, which also reads better in a screen reader's list of editor tabs. `cargo xtask ci` refuses a `mod.rs`.
3. **Folders are named by role**: `domain/`, `use_cases/`, `adapters/`. There is no `utils.rs` or `helpers.rs`.
4. **A module with submodules decides its public face**: its submodules are private, and it re-exports what the rest of the crate uses.
5. **Unit tests live with the code**, in `module/tests.rs`, declared with `#[cfg(test)] mod tests;` at the bottom of `module.rs`, in the module whose code decides what they check. Tests in `tests/` reach the crate only through its public API, as a user does.
6. **A file holds one thing, and its length is a signal.** A file holding two things or more, at any length, keeps one and moves each other to a private submodule. Past 200 lines `cargo xtask ci` fails, and the file splits along its seams. Each file is counted on its own, and a module's tests, in their own `tests.rs`, are counted apart from it.
7. **Import a symbol, then use it by its bare name**, except a crate's types used once where the crate's name says what they are, such as `cargo_metadata::Message` in an adapter.
8. **Helpers that the tests of several modules share** live in a test-only module, `test_support.rs` at the crate's root, never copied from one test module to another. Its role line says **Test support**.

## Writing the code

- **Formatting is plain `rustfmt`.**
- **Nothing panics.** Every failure is an `Error`. Clippy denies `unwrap_used`, `expect_used`, `panic`, `indexing_slicing`, `todo` and `unimplemented`, as `[workspace.lints]` sets, and `clippy.toml` allows the first four in tests.
- **No `unsafe` code**: `[workspace.lints]` forbids it.
- **Reuse before writing.** Look first at the standard library, then at the crates already depended on, then at well-known crates. Write it ourselves only when none fits, or when a new crate would replace only a couple of lines of logic; code a crate writes at no cost, as a derive or `getset` and `delegate` do, is never too little to leave to it. A crate is judged by what it replaces across the code, not only by its first use.
- **Derive what a derive produces.** Errors derive with `thiserror`; `Display` with `derive_more`; an enum's `is_*` predicates with `strum`'s `EnumIs`. An impl is written by hand only when it needs logic.
- **No code a crate writes for free.** When a crate already depended on produces code at no cost, the code is not written by hand, however simple: one getter is easy to read, but hundreds of getters, setters and forwarding methods are code every reader has to go through. A getter or setter is derived with `getset`, naming the field after the getter it gives and documenting the field, which becomes the getter's documentation; a method that only forwards to a field, such as `self.0.get(name)`, or a trait method that does, such as `Borrow::borrow`, is written with `delegate`. A hand-written one stays only when it does more than return or forward, and the reviewer judges each one.
- **Types hold every constraint they can.** An enum for a closed set of values, never a string or a `bool` standing for one, such as `Edition`, `Outcome`, `Level` and `DefaultFeatures`. A newtype for a name with rules of its own, such as `SnippetName`. A doc comment stating a constraint the type could enforce means the type is wrong. A newtype never derives `Deref`, which is for smart pointers.
- **A chain of iterator adapters reads as a sentence of names.** A closure in a chain is one call, one method call, one field read or one comparison with a value it captures; a closure that negates, matches, computes or builds a value becomes a named function.
- **Iterating to get a result uses adapters, not a `for` loop.** A `for` loop stays only when its body acts on each item, such as writing each file or asserting in a test.
- **Every line does something.** No line that changes nothing.

## Errors

- **One enum, `Error`**, in `domain/errors.rs`, derived with `thiserror`. Its `Display` is the sentence the user reads, so all of an error's words are in one place. Words from outside, such as the reason the system gave for a file it could not read or what Cargo wrote to its error output, are carried as they came.
- **Every message must make sense spoken by a screen reader**: words, never arrows or chains of symbols. A message names the snippet, the file or the dependency it is about.
- **Every problem with a description at once.** The naming policy returns `Errors`, which is never empty, gathering every problem instead of stopping at the first. Compiling stops at the first problem, since what comes after it needs what failed.

## The public API

- **Its names are a promise.** `compile`, `Compilation` and its methods, `Dependency`, `Edition`, `Outcomes`, `Outcome`, `Diagnostic`, `Location`, `SnippetName`, `Errors` and `Error` are what users write; changing how any of them is named or typed is agreed with the maintainer first.
- **Room to grow.** `Diagnostic`, `Location`, `Edition` and `Error` are `#[non_exhaustive]`, and so is each variant of `Error` with fields, so a later version can add a field, an edition or an error without breaking a user's code.

## Comments

A comment says only what the code cannot:

- the module's role line;
- an invariant or a hazard the code does not enforce itself;
- what an absent value, an error or a rejection carries.

One sentence, where it applies, once. Why a decision was taken, and what else was considered, belong in the pull request. A field, a function or a test whose name already says what it is gets no doc comment, except on the public API, whose every item says what it is for a user reading its documentation. No `TODO` comments. A measured fact names the program and its version, never a date. Code never cites an issue or a pull request.

## Tests

Cheapest first:

1. **Policy tests**, pure and in memory: the names, the project and the outcomes.
2. **Service tests**, the service over fake ports, checking the order of its steps and the errors it stops at.
3. **Adapter tests**: the Cargo adapter's translation of messages Cargo wrote, from their JSON; the disk adapter writing a project into a temporary folder.
4. **Gherkin scenarios**, `tests/features/`, which compile real snippets with the real Cargo against small dependencies of their own, `greeting` and `broken`, and one from crates.io, resolved with a lockfile of their own that pins an older version, and check what a user reads in the outcomes.

How a test is written:

- **A test's name states the rule it checks**, as a sentence, such as `a_snippet_without_its_crate_and_without_errors_has_no_outcome`.
- **A fixed bug gets a regression test that fails without the fix**, and a Gherkin scenario when a user can meet the bug.
- **A setup that three or more tests of one module start the same way** becomes a helper of those tests, named for what it makes.
- **A rule checked on several inputs is one test over a table of cases**, each case's input and expectation side by side.
