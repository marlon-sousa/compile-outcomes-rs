//! Entity: what the disk found at one place a compilation looks for snippets.

use std::collections::BTreeSet;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Found {
    /// A file given as a snippet.
    File(PathBuf),
    /// Every file of a folder given for its snippets.
    Folder(BTreeSet<PathBuf>),
    UnreadableSnippet {
        file: PathBuf,
        reason: String,
    },
    UnreadableFolder {
        folder: PathBuf,
        reason: String,
    },
}

impl Found {
    pub(crate) fn is_unreadable(&self) -> bool {
        matches!(
            self,
            Self::UnreadableSnippet { .. } | Self::UnreadableFolder { .. }
        )
    }
}
