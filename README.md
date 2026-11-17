# compile-outcomes

Compiles Rust snippets and reports what the compiler said about each one, without judging, for `#[test]`, Gherkin or any test framework.

compile-outcomes is made for test suites that check what a library tells its users when they misuse it, such as the errors of a procedural macro. You give it a set of Rust files, the snippets, and the crates they depend on. It compiles them all in one go with Cargo and gives you, for each snippet, whether it compiled and every error and warning, each with its message, its code and where it points. It asserts nothing: whether a snippet should have compiled, and which message it should show, is for your tests to judge, in whatever framework they use: a plain `#[test]`, Gherkin scenarios with the [cucumber](https://crates.io/crates/cucumber) crate, a snapshot tool, or any other.

## Why not trybuild?

[trybuild](https://crates.io/crates/trybuild) is excellent, and it is the right choice when what you want is a snapshot: it compares what the compiler wrote for each test case with a saved `.stderr` file, and rewrites the paths and details that differ between machines so the files stay stable. Much of the Rust ecosystem tests its macros with it.

compile-outcomes answers a different need. It hands your tests what the compiler said, as data: for each snippet, whether it compiled, and each error and warning with its message, its code and the line and column it points at. It judges nothing, so your test framework decides what matters, whether that is one error code, a few words of a message, or the whole text. And since it never compares whole texts, a test that checks codes and locations survives most compiler upgrades.

## Using it

Add it to the development dependencies of the crate whose tests use it:

```toml
[dev-dependencies]
compile-outcomes = "0.1"
```

Then describe a compilation, compile it, and look at the outcomes:

```rust,no_run
use compile_outcomes::{Compilation, Dependency, Edition, Errors, compile};

#[test]
fn a_struct_without_a_name_is_refused() -> Result<(), Errors> {
    let work_dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("missing_field");
    let compilation = Compilation::new(Edition::E2021, work_dir)
        .snippets_in("tests/ui")
        .dependency(Dependency::path("my_macros", "."));

    let outcomes = compile(&compilation)?;

    let outcome = outcomes.get("missing_field").expect("tests/ui/missing_field.rs");
    assert!(outcome.is_failed());
    let error = &outcome.errors()[0];
    assert_eq!(error.message, "missing field `name` in initializer of `User`");
    assert_eq!(error.code.as_deref(), Some("E0063"));
    assert_eq!(error.location.as_ref().map(|location| location.line), Some(4));
    Ok(())
}
```

### In Gherkin scenarios

Since compile-outcomes only reports, it fits any way of writing tests. With Gherkin, a feature file says what your macros refuse, in words a reader recognises, such as `tests/features/refusals.feature`:

```gherkin
Feature: What my macros refuse

  Scenario: A user without a name is refused
    Given the snippet "missing_field"
    When it is compiled
    Then it fails with an error that contains "missing field `name`"

  Scenario: A user without an age is refused
    Given the snippet "missing_age"
    When it is compiled
    Then it fails with an error that contains "missing field `age`"
```

Every snippet of `tests/ui` compiles once, the first time a scenario asks, and each scenario only looks its snippet up, so the scenarios run side by side. The steps, in `tests/ui.rs`:

```rust,ignore
use std::path::Path;
use std::sync::LazyLock;

use compile_outcomes::{Compilation, Edition, Outcome, Outcomes, compile};
use cucumber::{World, given, then, when};

/// Every snippet of tests/ui, compiled once, the first time a scenario asks.
static OUTCOMES: LazyLock<Outcomes> = LazyLock::new(|| {
    let work_dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("ui");
    let compilation = Compilation::new(Edition::E2021, work_dir).snippets_in("tests/ui");
    compile(&compilation).expect("the compilation is well described")
});

#[derive(Debug, Default, World)]
struct Ui {
    snippet: String,
    outcome: Option<&'static Outcome>,
}

#[given(expr = "the snippet {string}")]
fn the_snippet(ui: &mut Ui, name: String) {
    ui.snippet = name;
}

#[when("it is compiled")]
fn it_is_compiled(ui: &mut Ui) {
    ui.outcome = OUTCOMES.get(&ui.snippet);
}

#[then(expr = "it fails with an error that contains {string}")]
fn fails_with(ui: &mut Ui, words: String) {
    let outcome = ui.outcome.expect("the snippet is in tests/ui");
    assert!(outcome.is_failed(), "{outcome:#?}");
    let said = outcome.errors().iter().any(|error| error.message.contains(&words));
    assert!(said, "{outcome:#?}");
}

fn main() {
    futures::executor::block_on(Ui::run("tests/features"));
}
```

The scenarios run as a test of their own, without the default test harness, with `cucumber` and `futures` among the development dependencies:

```toml
[[test]]
name = "ui"
harness = false
```

### Describing a compilation

- `Compilation::new(edition, work_dir)` starts one. `work_dir` is a folder compile-outcomes owns: it writes the project there and keeps the build output between compilations, unless the build is clean, so the dependencies compile once. A folder under your `target/` suits it, such as one in the `CARGO_TARGET_TMPDIR` Cargo gives integration tests. compile-outcomes marks the project folder it writes there with a file, `.owned-by-compile-outcomes`, and never replaces a folder holding other files without it; create that file to hand such a folder over. Two compilations must not share a work folder at the same time, and tests run in parallel, so give each test that compiles its own.
- `.snippet(path)` adds one snippet, and `.snippets_in(dir)` every `.rs` file of a folder, not looking into subfolders.
- `.dependency(Dependency::path(name, path))` or `.dependency(Dependency::version(name, "1.0"))` gives every snippet a dependency, with `.features([...])` and `.without_default_features()` when needed.
- `.lockfile("Cargo.lock")` makes the dependencies resolve to the versions your own lockfile holds. It is copied, never written.
- `.clean_build()` runs `cargo clean` on the build output first, so that every crate compiles again, dependencies included. It cleans only a target folder Cargo marked as its own with a `CACHEDIR.TAG` file.

A relative path is taken from the current folder, which for a test is its crate's folder.

Describing a compilation touches nothing on disk; only `compile` does.

### Snippets

Each snippet is the root of its own library crate, so it needs no `main`. It is named after its file: `tests/ui/missing_field.rs` is the snippet `missing_field`, whose crate is `snippet_missing_field`. A name is made of ASCII letters, digits, `_` and `-`, and does not start with a digit.

### Outcomes

`compile` returns the `Outcomes`, one `Outcome` for each snippet, looked up with `.get(name)` or iterated in the order of the names:

- `Outcome::Compiled { warnings }` when Cargo produced the snippet's crate;
- `Outcome::Failed { errors, warnings }` when the compiler reported errors for it.

Each `Diagnostic` has its `message`, the first line; its `code`, such as `E0425` or `dead_code`; its `location`, the file, line and column it points at; and `rendered`, the whole message as a terminal shows it. A snippet that fails to compile is an outcome, never an error.

### Errors

`compile` returns `Errors` when something kept it from the outcomes. Every problem with the description, such as a snippet whose name is not valid or two snippets of the same name, is reported together. A problem while compiling, such as a dependency that fails to compile or Cargo that cannot be started, stops the compilation and is reported alone. Each error is a sentence naming what it is about, and `Errors` displays as those sentences, each starting on a line of its own.

### Which Cargo runs

compile-outcomes runs the `cargo` the `CARGO` environment variable names, which Cargo sets for the tests it runs, so your snippets compile with the same toolchain as your tests. Otherwise it runs the `cargo` on the `PATH`. Cargo works as configured for you: its network access, registries and environment variables apply.

### Platforms

compile-outcomes works the same on Linux, macOS and Windows, and its tests run on all three. It compares no text, so it rewrites none: a diagnostic's `location.file` is the path the compiler gave, with the platform's separators, and its `message` and `rendered` are the compiler's own words. A test that compares them with text it holds, the same on every platform, makes paths alike first, for example by replacing each `\` with `/`.

### Compiler versions

Compiler messages change from one Rust version to the next, and compile-outcomes reports them as they are. A diagnostic's `code`, such as `E0425`, and its `line` and `column` rarely change; its `message`, one line, sometimes does; and `rendered`, the whole block, changes most often. A suite that checks codes and locations, and messages only where they matter, survives most upgrades; one that compares `rendered` runs best on one pinned toolchain.

### What 0.1.0 does not do yet

Snippets compiled as binaries or tests, dependencies by git URL or for only some snippets, choosing the target, the profile or extra `rustc` flags, a time limit, and diagnostics with their paths rewritten alike on every platform are not part of 0.1.0. [docs/SPEC.md](docs/SPEC.md) describes exactly what it does.

The minimum supported Rust version is 1.85.

## Contributing

compile-outcomes is built in small, stacked pull requests, each reviewed against written rules:

- [docs/SPEC.md](docs/SPEC.md) says what compile-outcomes does, accepts and refuses. A change to what it does starts there.
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) says how the code is organised and written: a pure domain that decides, and adapters at the edges that write files and run Cargo.
- [docs/WORKFLOW.md](docs/WORKFLOW.md) says how a change travels to `main`, what each check does, and how a release is made.
- [AGENTS.md](AGENTS.md) holds the rules every change follows and how it is reviewed, for people and agents alike.

Before pushing, run the quality gate from the repository's root:

```sh
cargo xtask ci
```

It checks formatting, clippy with every warning an error, the architecture, every unit test and Gherkin scenario, the documentation, the dependencies with cargo-deny, and a publish dry run.

## Licence

Licensed under either the [Apache License, Version 2.0](LICENSE-APACHE) or the [MIT licence](LICENSE-MIT), at your option.
