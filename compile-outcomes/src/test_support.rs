//! Test support: the values the tests of several modules make the same way.

use crate::domain::{Diagnostic, SnippetName};

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
