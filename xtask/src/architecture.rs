//! Policy: the architecture check, what docs/ARCHITECTURE.md asks of every file that a tool can see.

mod input_output;

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// The folders the check reads, relative to the workspace's root.
pub(crate) const FOLDERS: [&str; 2] = ["compile-outcomes/src", "xtask/src"];

/// The folder whose `domain` and `use_cases` modules do no input or output.
const LIBRARY: &str = "compile-outcomes/src";

const PURE_MODULES: [&str; 2] = ["domain", "use_cases"];

const LINE_LIMIT: usize = 200;

/// A Rust file, with its path relative to the workspace's root, and its text or the reason it could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Source {
    pub(crate) path: PathBuf,
    pub(crate) text: Result<String, String>,
}

/// Every rule `source` breaks, each a sentence naming its file.
pub(crate) fn breaches(source: Source) -> Vec<String> {
    let name = source.path.display().to_string();
    let text = match source.text {
        Ok(text) => text,
        Err(reason) => return vec![format!("{name} could not be read: {reason}")],
    };
    let input_output = if is_pure(&source.path) {
        input_output::breaches(&text)
    } else {
        Ok(Vec::new())
    };
    let input_output = match input_output {
        Ok(breaches) => breaches,
        Err(reason) => vec![format!("could not be parsed, {reason}")],
    };
    layout_breaches(&source.path, &text)
        .into_iter()
        .chain(input_output)
        .map(|breach| located(&name, breach))
        .collect()
}

/// Whether `path` is part of a module that does no input or output.
fn is_pure(path: &Path) -> bool {
    let Ok(inner) = path.strip_prefix(LIBRARY) else {
        return false;
    };
    let Some(first) = inner.components().next() else {
        return false;
    };
    let module = Path::new(first.as_os_str()).file_stem();
    PURE_MODULES
        .iter()
        .map(OsStr::new)
        .any(|pure| module == Some(pure))
}

fn located(name: &str, breach: String) -> String {
    format!("{name}: {breach}")
}

fn layout_breaches(path: &Path, text: &str) -> Vec<String> {
    let lines = text.lines().count();
    let mod_rs = (path.file_name().is_some_and(|name| name == "mod.rs"))
        .then_some("it is a mod.rs; name the file after its module instead.".to_owned());
    let length = (lines > LINE_LIMIT).then_some(format!(
        "it has {lines} lines, more than {LINE_LIMIT}; split it along its seams."
    ));
    mod_rs.into_iter().chain(length).collect()
}

#[cfg(test)]
mod tests;
