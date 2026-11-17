//! Policy: whose a folder compile-outcomes would replace or remove is, from the names it holds.

use std::collections::BTreeSet;
use std::ffi::{OsStr, OsString};

use crate::domain::{CARGO_MARKER, OWNER_MARKER, OwnedFolder};

/// What a project folder holding `names` is, `None` meaning there is no folder yet.
pub(crate) fn project_folder(names: Option<&BTreeSet<OsString>>) -> OwnedFolder {
    marked_with(OWNER_MARKER, names)
}

/// What a target folder holding `names` is, `None` meaning there is no folder yet.
pub(crate) fn target_folder(names: Option<&BTreeSet<OsString>>) -> OwnedFolder {
    marked_with(CARGO_MARKER, names)
}

fn marked_with(marker: &str, names: Option<&BTreeSet<OsString>>) -> OwnedFolder {
    match names {
        None => OwnedFolder::Missing,
        Some(names) if names.is_empty() => OwnedFolder::Empty,
        Some(names) if names.contains(OsStr::new(marker)) => OwnedFolder::Owned,
        Some(_) => OwnedFolder::Foreign,
    }
}

#[cfg(test)]
mod tests;
