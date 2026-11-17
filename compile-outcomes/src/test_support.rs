//! Test support: the values the tests of several modules make the same way.

use std::path::PathBuf;

use crate::domain::{Diagnostic, Snippet, SnippetName};

pub(crate) fn name(name: &str) -> SnippetName {
    SnippetName::parse(name).unwrap()
}

pub(crate) fn diagnostic(message: &str) -> Diagnostic {
    Diagnostic {
        message: message.to_owned(),
        code: None,
        location: None,
        rendered: format!("error: {message}"),
    }
}

pub(crate) fn snippet(name_of_snippet: &str, file: &str) -> Snippet {
    Snippet {
        name: name(name_of_snippet),
        file: PathBuf::from(file),
    }
}
