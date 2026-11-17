//! Policy: the snippets of a compilation, named after their files, with every problem with them.

use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::domain::{Error, Errors, Found, Snippet, SnippetName};

/// The snippets of what the disk found, in the order found, or every problem with them.
pub(crate) fn snippets(found: Vec<Found>) -> Result<Vec<Snippet>, Errors> {
    let every_place_read = !found.iter().any(Found::is_unreadable);
    let mut errors = Vec::new();
    let files: Vec<PathBuf> = found
        .into_iter()
        .flat_map(|found| snippet_files(found, &mut errors))
        .collect();
    if files.is_empty() && every_place_read {
        errors.push(Error::NoSnippet);
    }
    let named: Vec<Snippet> = files
        .into_iter()
        .filter_map(|file| named(file, &mut errors))
        .collect();
    let snippets = distinct(named, &mut errors);
    Errors::gathered(errors).map_or(Ok(snippets), Err)
}

fn snippet_files(found: Found, errors: &mut Vec<Error>) -> Vec<PathBuf> {
    match found {
        Found::File(file) => vec![file],
        Found::Folder(files) => files
            .into_iter()
            .filter(|file| is_rust_source(file))
            .collect(),
        Found::UnreadableSnippet { file, reason } => {
            errors.push(Error::UnreadableSnippet { file, reason });
            Vec::new()
        }
        Found::UnreadableFolder { folder, reason } => {
            errors.push(Error::UnreadableFolder { folder, reason });
            Vec::new()
        }
    }
}

fn is_rust_source(file: &Path) -> bool {
    file.extension() == Some(OsStr::new("rs"))
}

fn named(file: PathBuf, errors: &mut Vec<Error>) -> Option<Snippet> {
    match name_of(&file) {
        Some(name) => Some(Snippet { name, file }),
        None => {
            errors.push(Error::InvalidName { file });
            None
        }
    }
}

fn name_of(file: &Path) -> Option<SnippetName> {
    file.file_stem()
        .and_then(OsStr::to_str)
        .and_then(SnippetName::parse)
}

/// The snippets whose crate names no earlier snippet took, with an error for each other.
fn distinct(snippets: Vec<Snippet>, errors: &mut Vec<Error>) -> Vec<Snippet> {
    let mut taken = BTreeMap::new();
    snippets
        .into_iter()
        .filter_map(|snippet| first_of_its_name(snippet, &mut taken, errors))
        .collect()
}

/// The snippet when its crate name is still free, taking it; otherwise `None`, with the error naming both files.
fn first_of_its_name(
    snippet: Snippet,
    taken: &mut BTreeMap<String, PathBuf>,
    errors: &mut Vec<Error>,
) -> Option<Snippet> {
    match taken.entry(snippet.name.crate_name()) {
        Entry::Vacant(free) => {
            free.insert(snippet.file.clone());
            Some(snippet)
        }
        Entry::Occupied(first) => {
            errors.push(Error::DuplicateName {
                crate_name: first.key().clone(),
                first: first.get().clone(),
                second: snippet.file,
            });
            None
        }
    }
}

#[cfg(test)]
mod tests;
