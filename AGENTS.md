# AGENTS.md: working in compile-outcomes

The manual for anyone, human or agent, changing compile-outcomes. [docs/WORKFLOW.md](docs/WORKFLOW.md) describes the stacked pull requests and the definition of done; this file says what every change follows and how it is reviewed.

## What to read

- [docs/SPEC.md](docs/SPEC.md): what compile-outcomes does, accepts and refuses, and what ships in 0.1.0. Code does what it says, no more and no less.
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md): how the code is organised and written, before writing any.
- [.claude/skills/](.claude/skills/): the procedures, such as having a change reviewed. Claude Code loads them on its own; another agent reads the one it needs.
- The pull request or issue the work belongs to. Once a decision is agreed there, it binds the work.

## Rules

1. **No code before its behaviour.** A change to what compile-outcomes does, accepts or refuses starts in docs/SPEC.md. If the code would need to do something SPEC.md does not say, stop and ask the maintainer.
2. **Every behaviour has a test a user would recognise.** A behaviour added to or changed in SPEC.md comes with a Gherkin scenario under `tests/features/`, in the same pull request as the code, unless only a fake port can reach it, such as Cargo failing to start; then a service test checks it, and the pull request says why.
3. **A fixed bug gets a regression test that fails without the fix**, and a Gherkin scenario, tagged `@regression`, when a user can meet the bug.
4. **Errors are for people.** Every error is a sentence that reads well spoken by a screen reader, names what it is about, and is reported together with every other problem found, as docs/ARCHITECTURE.md, "Errors", says.
5. **The public API is a promise.** Never change how a public item is named or typed, such as `compile`, `Compilation` or `Outcome`, without the maintainer's agreement.
6. **Small pull requests.** One thing each, stacked as docs/WORKFLOW.md describes, with Conventional Commits messages carrying the changelog trailers docs/WORKFLOW.md asks for.
7. **Write for a screen reader.** Pull requests, comments and documents use headings, lists, prose and simple tables, never ASCII art. A diagram is Mermaid with `accTitle` and `accDescr`.
8. **Mark agent comments.** A comment an agent writes on GitHub says it is an agent's.
9. **Check a library before ruling it in or out.** Before saying that a crate or a tool can or cannot do something, read its source or its documentation at the version we use, including its extension points. The pull request says what was checked: the file and line, or the documentation page.
10. **One question at a time.** A question for the maintainer is asked in plain prose, one decision at a time, with the option recommended and why.

## Reviewing a change

Every pull request is reviewed by an independent reviewer before it is opened, one given the change, not its author's reasoning. A later push to an open pull request is reviewed the same way while its CI runs. An agent's `.claude/skills/review-change` says how to start and resume the reviewer. The reviewer is given this section in full, the documents under "What to read", the commits to review and the draft pull request description. For each item, it lists every place in the diff where the item applies, not only the first, and checks each against the rule it names.

1. Every behaviour the change adds or changes, against docs/SPEC.md, and against rules 1 to 3 above: its scenario, its service test, its regression test.
2. Every error the change adds or changes, against rule 4 above and docs/ARCHITECTURE.md, "Errors": its words, what it names, and that it is reported with the other problems found where the description has several.
3. Every public item the change adds or changes, against rule 5 above and docs/ARCHITECTURE.md, "The public API": its name, its documentation, and whether it should be `#[non_exhaustive]`.
4. `compile-outcomes/src/lib.rs`, against docs/ARCHITECTURE.md, "Folders and files", rule 1: no bare `pub` other than the agreed public API.
5. Every file the change adds or makes longer, against docs/ARCHITECTURE.md, "Folders and files", rule 6: what it holds, and whether it holds one thing well; if not, the private submodules it splits into.
6. Every new module: its role line, its folder and its name, against docs/ARCHITECTURE.md, "Module roles" and "Folders and files".
7. Every module of `domain/`, `use_cases/`, `ports.rs` and `adapters/`, against docs/ARCHITECTURE.md, "Module roles" and "The typed model": a decision an adapter takes, a call that goes up a layer, input or output in `domain/` or `use_cases/`, a lookup by name that could fail, and a port around a pure function are findings.
8. Against docs/ARCHITECTURE.md, "Writing the code": every `unwrap`, `expect`, `panic!`, indexing, `todo!` and `unimplemented!` outside tests; every hand-written trait impl and `new`, naming the derive that would produce it or why none does; every hand-written getter, setter or method that only forwards to a field, and any other code a crate already depended on would write at no cost, naming that crate; every helper, naming what already provides it, if anything; every string or `bool` standing for a closed set, and every constraint stated in a comment that a type could hold; every newtype deriving `Deref`; every closure in a chain of adapters and every `for` loop; every line that changes nothing.
9. Every test the change adds or moves, against docs/ARCHITECTURE.md, "Folders and files", rule 5, and "Tests": where it lives, what its name states, and that a regression test fails without its fix.
10. Every comment and doc comment, against docs/ARCHITECTURE.md, "Comments".
11. Every crate the change adds, removes or uses in a new way, and every claim that a library or a tool can or cannot do something, against rule 9 above: it names the source it was checked against.
12. Every statement of docs/SPEC.md, docs/ARCHITECTURE.md, docs/WORKFLOW.md and README.md the change makes untrue, which the change updates itself.
13. The commit messages and the description, against rule 6 above and docs/WORKFLOW.md; for a layer of a stack, that its branch sits on the current tip of the layer below.
14. The checks under "Building and testing", run at the reviewed revision, without changing the checkout the author works in.

The report lists findings, each marked blocking or not, with its `file:line` and a fix, then a verdict: approve, or changes needed. A finding the author rejects goes back to the reviewer with the reason, and a blocking one closes only when the reviewer agrees or the maintainer decides. A pull request is reported ready to the maintainer only once both the review and the CI of its latest push have passed.

## Building and testing

The toolchain is pinned in `rust-toolchain.toml`. Before pushing, run `cargo xtask ci` from the repository's root: formatting, clippy with every warning an error, the architecture check (no input or output in `domain/` or `use_cases/`, no `mod.rs`, files of at most 200 lines), the README's examples against the files that hold them, every unit test and Gherkin scenario, the documentation, the dependencies with cargo-deny, and a publish dry run. docs/WORKFLOW.md, section 7, lists each check and how to install cargo-deny.
