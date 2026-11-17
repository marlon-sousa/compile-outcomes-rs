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
    /// The check that the README's marked examples are what their files hold.
    Readme,
}

/// The modes `argument` names: every mode for `ci` or for none, otherwise the one of that name.
pub(crate) fn requested(argument: Option<&str>) -> Option<Vec<Mode>> {
    match argument {
        None | Some("ci") => Some(Mode::iter().collect()),
        Some(name) => Mode::from_str(name).ok().map(alone),
    }
}

fn alone(mode: Mode) -> Vec<Mode> {
    vec![mode]
}

/// The step `mode` runs.
pub(crate) fn step(mode: Mode) -> Step {
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
        Mode::Readme => Step::Readme,
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
