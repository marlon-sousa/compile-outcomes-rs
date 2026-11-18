//! Adapter: the Rust files of the checked folders, and their text, as the architecture check reads them.

use std::fs;
use std::path::{Path, PathBuf};

use walkdir::{DirEntry, Error, WalkDir};

use crate::architecture::Source;

/// Every `.rs` file under each of `folders`, in subfolders too, sorted, with its path relative to `root`; a link is followed.
///
/// A folder or a file that cannot be read is a source whose text is the reason the system gave.
pub(crate) fn read(root: &Path, folders: &[&str]) -> Vec<Source> {
    folders
        .iter()
        .map(|folder| root.join(folder))
        .flat_map(|folder| sources_under(root, folder))
        .collect()
}

fn sources_under(root: &Path, folder: PathBuf) -> Vec<Source> {
    WalkDir::new(&folder)
        .follow_links(true)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|entry| source(root, &folder, entry))
        .collect()
}

fn source(root: &Path, folder: &Path, entry: Result<DirEntry, Error>) -> Option<Source> {
    match entry {
        Ok(entry) if is_rust_file(&entry) => Some(Source {
            path: relative(root, entry.path()),
            text: fs::read_to_string(entry.path()).map_err(|error| error.to_string()),
        }),
        Ok(_) => None,
        Err(error) => Some(Source {
            path: relative(root, error.path().unwrap_or(folder)),
            text: Err(reason(&error)),
        }),
    }
}

/// The reason the system gave, without the path walkdir adds to it, or walkdir's own words for a link loop.
fn reason(error: &Error) -> String {
    error
        .io_error()
        .map_or_else(|| error.to_string(), ToString::to_string)
}

fn is_rust_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
        && entry
            .path()
            .extension()
            .is_some_and(|extension| extension == "rs")
}

fn relative(root: &Path, path: &Path) -> PathBuf {
    path.strip_prefix(root).unwrap_or(path).to_owned()
}
