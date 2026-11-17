//! Policy: whose the folder a project is written to is, from the names it holds.

use std::collections::BTreeSet;
use std::ffi::{OsStr, OsString};

use crate::domain::{OWNER_MARKER, ProjectFolder};

/// What a project folder holding `names` is, `None` meaning there is no folder yet.
pub(crate) fn project_folder(names: Option<&BTreeSet<OsString>>) -> ProjectFolder {
    match names {
        None => ProjectFolder::Missing,
        Some(names) if names.is_empty() => ProjectFolder::Empty,
        Some(names) if names.contains(OsStr::new(OWNER_MARKER)) => ProjectFolder::Owned,
        Some(_) => ProjectFolder::Foreign,
    }
}

#[cfg(test)]
mod tests;
