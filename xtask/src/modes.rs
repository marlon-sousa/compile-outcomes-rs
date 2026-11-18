//! Policy: the modes of the gate, and the steps each one runs.

use std::str::FromStr;

use strum::{Display, EnumIter, EnumString, IntoEnumIterator, VariantNames};

/// The modes `cargo xtask ci` runs, in this order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter, EnumString, VariantNames)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum Mode {
    Fmt,
    Clippy,
    Arch,
    Readme,
    Test,
    Doc,
    Deny,
    Package,
}

/// One step of a mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Step {
    /// Cargo, with these arguments.
    Cargo(&'static [&'static str]),
    /// Cargo, with these arguments, and every warning of rustdoc an error.
    Rustdoc(&'static [&'static str]),
    /// The architecture check.
    Architecture,
    /// The check that the README's marked examples are what their files hold, and the crate's README, checked or written.
    Readme(CrateReadme),
}

/// What the `readme` mode does with the crate's README.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CrateReadme {
    /// Refuses one that is not what the root README gives.
    Check,
    /// Writes it from the root README, as `cargo xtask readme --write` asks.
    Write,
}

/// What the command line asks of the gate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Request {
    pub(crate) modes: Vec<Mode>,
    pub(crate) crate_readme: CrateReadme,
}

/// What `arguments` ask: every mode for `ci` or for none, the one mode they name, or `readme --write`; nothing for any other.
pub(crate) fn requested(arguments: &[&str]) -> Option<Request> {
    match arguments {
        [] | ["ci"] => Some(checking(Mode::iter().collect())),
        ["readme", "--write"] => Some(Request {
            modes: vec![Mode::Readme],
            crate_readme: CrateReadme::Write,
        }),
        [name] => Mode::from_str(name).ok().map(alone),
        _ => None,
    }
}

fn alone(mode: Mode) -> Request {
    checking(vec![mode])
}

fn checking(modes: Vec<Mode>) -> Request {
    Request {
        modes,
        crate_readme: CrateReadme::Check,
    }
}

/// The step `mode` runs, doing `crate_readme` with the crate's README.
pub(crate) fn step(mode: Mode, crate_readme: CrateReadme) -> Step {
    match mode {
        Mode::Fmt => Step::Cargo(&["fmt", "--all", "--", "--check"]),
        Mode::Clippy => Step::Cargo(&[
            "clippy",
            "--workspace",
            "--all-targets",
            "--locked",
            "--",
            "-D",
            "warnings",
        ]),
        Mode::Arch => Step::Architecture,
        Mode::Readme => Step::Readme(crate_readme),
        Mode::Test => Step::Cargo(&["test", "--workspace", "--all-targets", "--locked"]),
        Mode::Doc => Step::Rustdoc(&["doc", "--workspace", "--no-deps", "--locked"]),
        Mode::Deny => Step::Cargo(&["deny", "check"]),
        Mode::Package => Step::Cargo(&[
            "publish",
            "--workspace",
            "--dry-run",
            "--locked",
            "--allow-dirty",
            "--target-dir",
            "target/package-check",
        ]),
    }
}

#[cfg(test)]
mod tests;
