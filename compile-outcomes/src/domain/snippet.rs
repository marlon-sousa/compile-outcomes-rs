//! Entity: a snippet, the root of a library crate, named after its file.

use std::path::PathBuf;

use crate::domain::SnippetName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Snippet {
    pub(crate) name: SnippetName,
    pub(crate) file: PathBuf,
}
