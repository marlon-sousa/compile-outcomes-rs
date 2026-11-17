//! Adapter: the Rust files of the checked folders, and their text, as the architecture check reads them.

use std::ffi::OsString;
use std::fs::{self, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

use crate::architecture::Source;

/// Every `.rs` file under each of `folders`, in subfolders too, sorted, with its path relative to `root`.
///
/// A folder or a file that cannot be read is a source whose text is the reason the system gave.
pub(crate) fn read(root: &Path, folders: &[&str]) -> Vec<Source> {
    folders
        .iter()
        .map(Path::new)
        .flat_map(|folder| under(root, folder))
        .collect()
}

fn under(root: &Path, folder: &Path) -> Vec<Source> {
    match names_in(&root.join(folder)) {
        Ok(names) => names
            .iter()
            .map(|name| folder.join(name))
            .flat_map(|path| source_or_more(root, path))
            .collect(),
        Err(error) => vec![unreadable(folder, &error)],
    }
}

fn names_in(folder: &Path) -> io::Result<Vec<OsString>> {
    let mut names = fs::read_dir(folder)?
        .map(file_name)
        .collect::<io::Result<Vec<_>>>()?;
    names.sort();
    Ok(names)
}

fn file_name(entry: io::Result<DirEntry>) -> io::Result<OsString> {
    Ok(entry?.file_name())
}

fn source_or_more(root: &Path, path: PathBuf) -> Vec<Source> {
    if root.join(&path).is_dir() {
        under(root, &path)
    } else if path.extension().is_some_and(|extension| extension == "rs") {
        let text = fs::read_to_string(root.join(&path)).map_err(|error| error.to_string());
        vec![Source { path, text }]
    } else {
        Vec::new()
    }
}

fn unreadable(path: &Path, error: &io::Error) -> Source {
    Source {
        path: path.to_owned(),
        text: Err(error.to_string()),
    }
}
