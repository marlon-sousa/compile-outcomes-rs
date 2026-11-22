//! Adapter (Cargo): the arguments that hand clippy the lint levels of a `Clippy`, after `--`.

use crate::domain::{Clippy, LintLevel, Lints, Setting};

/// `--`, then a flag and a lint for each setting, in order, or nothing for clippy's own levels.
pub(super) fn flags(clippy: &Clippy) -> Vec<String> {
    let settings = clippy.settings();
    if settings.is_empty() {
        return Vec::new();
    }
    let separator = std::iter::once("--".to_owned());
    separator.chain(settings.iter().flat_map(setting)).collect()
}

fn setting(setting: &Setting) -> [String; 2] {
    let flag = match setting.level {
        LintLevel::Allow => "-A",
        LintLevel::Warn => "-W",
        LintLevel::Deny => "-D",
    };
    let lints = match &setting.lints {
        Lints::Group(group) => group.to_string(),
        Lints::One(name) => name.clone(),
    };
    [flag.to_owned(), format!("clippy::{lints}")]
}

#[cfg(test)]
mod tests;
