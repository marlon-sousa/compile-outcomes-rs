//! Entity: a snippet, the root of a library crate, named after its file.

use std::path::PathBuf;

use delegate::delegate;

use crate::domain::SnippetName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Snippet {
    pub(crate) name: SnippetName,
    pub(crate) file: PathBuf,
}

impl Snippet {
    delegate! {
        to self.name {
            pub(crate) fn crate_name(&self) -> String;
        }
    }
}
