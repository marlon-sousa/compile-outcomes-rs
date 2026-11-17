# compile-outcomes: specification

What compile-outcomes does, accepts and refuses. [ARCHITECTURE.md](ARCHITECTURE.md) says how the code that does it is organised; [AGENTS.md](../AGENTS.md) says how a change is made. Code does what this document says, no more and no less, and a change to what it does starts here.

## 0. What ships in 0.1.0

Everything this document describes outside this section ships in 0.1.0. These are not part of 0.1.0, and are listed so that a later version can add them without breaking anything here:

- snippets compiled as binaries or tests instead of libraries;
- dependencies given by git URL, and dependencies that only some snippets get;
- choosing the target triple, the profile or extra `rustc` flags;
- a time limit on a compilation.

## 1. Purpose

compile-outcomes compiles a set of Rust source files, called snippets, and reports what the compiler said about each one: whether it compiled, and every error and warning, each with its message, its code and where it points. It decides nothing about them. Whether a snippet should have compiled, or which message it should show, is for its caller to judge.

It is made for test suites that check what a library tells its users when they misuse it, such as the errors of a procedural macro, but it asserts nothing itself.

## 2. Describing a compilation

A `Compilation` describes one compilation. It is built step by step, and each step returns it:

1. `Compilation::new(edition, work_dir)` starts one. `edition` is an `Edition`: `E2015`, `E2018`, `E2021` or `E2024`. `work_dir` is a folder compile-outcomes owns: it writes the project it builds there, and reuses the build output there from one compilation to the next.
2. `.snippet(path)` adds the snippet in the file `path`, whatever its extension.
3. `.snippets_in(dir)` adds every file of `dir` whose extension is `rs`, not looking into subfolders.
4. `.dependency(dependency)` gives every snippet a dependency, a `Dependency`:
   - `Dependency::path(name, path)`, a crate in a folder, as `name = { path = "..." }` in a manifest;
   - `Dependency::version(name, requirement)`, a crate from crates.io, as `name = "requirement"`;
   - `.features([...])` enables those features of it;
   - `.without_default_features()` turns its default features off.
5. `.lockfile(path)` copies that `Cargo.lock` into the project before building, so that the dependencies resolve to the versions it holds, such as those of the caller's own workspace. Cargo still updates the copy where it must, and the original is never written.

Describing a compilation touches nothing on disk; only compiling it does.

## 3. Snippets

1. **A snippet is the root of a library crate.** It needs no `main` function, and one it has is compiled like any other function.
2. **A snippet is named after its file**, without the extension: `tests/ui/missing_field.rs` is the snippet `missing_field`. A name is made of ASCII letters, digits, `_` and `-`, and does not start with a digit.
3. **A snippet's crate is named `snippet_<name>`**, with every `-` written as `_`. So its own items are reached through `crate::`, and its name never clashes with a dependency's.
4. **Two snippets cannot share a name**, nor names that differ only by `-` and `_`, since their crates would.
5. **Each snippet is its own package** in one generated workspace, so the snippets compile in parallel and share the build of their dependencies.

## 4. Compiling

`compile_outcomes::compile(&compilation)` compiles every snippet and returns either the `Outcomes` or the `Errors` that kept it from them.

1. It writes the generated workspace to `work_dir/project`, replacing whatever an earlier compilation left there, and copies the lockfile into it when one was given.
2. It runs `cargo build --workspace --keep-going --message-format=json` there, with the target folder `work_dir/target`. `--keep-going` lets every snippet compile even when an earlier one failed.
3. It runs the `cargo` the `CARGO` environment variable names, which Cargo sets for the tests and build scripts it runs, so a suite compiles its snippets with its own toolchain; otherwise the `cargo` found on the `PATH`.
4. Cargo works as configured for the user: its network access, its registries and its environment variables apply.

## 5. Outcomes

1. **`Outcomes` holds one `Outcome` for every snippet**, looked up with `.get(name)`, where `name` is the snippet's name as a `&str`, and iterated, with `.iter()` or a `for` loop over `&outcomes`, as pairs of a `SnippetName` and its `Outcome`, in the order of the names. A `SnippetName` gives the name with `.as_str()`, and displays as it.
2. **An `Outcome` is `Compiled { warnings }` or `Failed { errors, warnings }`.** A snippet compiled when Cargo produced its crate; it failed when the compiler reported errors for it. `.is_compiled()` and `.is_failed()` tell which, and `.errors()` and `.warnings()` give them whatever the outcome, no errors for a snippet that compiled.
3. **A `Diagnostic` is one message of the compiler**, with:
   - `message`, its first line, such as ``cannot find function `shout` in crate `greeting` ``;
   - `code`, the error code or the lint it comes from, such as `E0425` or `dead_code`, when it has one;
   - `location`, where its first primary span points, when it points anywhere: the `file` as the compiler names it, and the `line` and `column`, counted from 1;
   - `rendered`, the whole message as a terminal shows it.
4. **Errors are the messages of level error**, an internal compiler error included; **warnings are those of level warning**. The compiler's closing notes, such as "For more information about an error, try `rustc --explain`", are neither, and are left out.
5. **Diagnostics keep the order the compiler gave them.**

## 6. Errors

`Errors` holds every problem that kept a compilation from its outcomes, in the order they were found, and is never empty: `.first()` gives the first, and `.iter()`, or a `for` loop over it, every one. Each `Error` is a sentence that names what it is about, and `Errors` reads as those sentences, each starting on a line of its own.

1. **Problems with the description are reported together**, before anything is written:
   - a compilation with no snippet;
   - a snippet whose name breaks §3, rule 2, naming its file;
   - two snippets with the same name, or names that differ only by `-` and `_`, naming both files;
   - a snippet file, or a folder given to `.snippets_in`, that cannot be read, naming which, with the reason the system gave.
2. **Problems while compiling stop it**, and are reported alone:
   - the project or the lockfile cannot be written, with the reason the system gave;
   - Cargo cannot be started, naming the program run, with the reason the system gave;
   - Cargo stops without building any snippet and without a compiler message, such as when a dependency cannot be found or resolved, with what it wrote to its error output;
   - a dependency fails to compile, naming it, with its errors;
   - Cargo finishes without an outcome for a snippet, naming it.

A snippet that fails to compile is an outcome, never an error.
