//! Adapter (Cargo): the arguments that hand clippy the lint levels of a `Clippy`, after `--`.

use std::iter::once;

use crate::domain::{Clippy, LintLevel, Lints, Setting};

/// `--`, then a flag and a lint for each setting, in order.
pub(super) fn flags(clippy: &Clippy) -> Vec<String> {
    once("--".to_owned())
        .chain(clippy.settings().iter().flat_map(setting))
        .collect()
}

fn setting(setting: &Setting) -> [String; 2] {
    let flag = match setting.level {
        LintLevel::Allow => "-A",
        LintLevel::Warn => "-W",
        LintLevel::Deny => "-D",
    };
    let lints = match &setting.lints {
        Lints::Group(group) => group.to_string(),
        Lints::One(name) => name.to_string(),
    };
    [flag.to_owned(), format!("clippy::{lints}")]
}

#[cfg(test)]
mod tests;
