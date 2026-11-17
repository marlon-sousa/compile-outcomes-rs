//! Entity: the name of a snippet, which is known to be valid.

use std::borrow::Borrow;

use derive_more::Display;

/// The name of a snippet: its file's name without the extension.
///
/// A name is made of ASCII letters, digits, `_` and `-`, and does not start with a digit.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct SnippetName(String);

impl SnippetName {
    /// The name, or `None` when `name` breaks the rules of a name.
    pub(crate) fn parse(name: &str) -> Option<Self> {
        let starts_well = name.chars().next().is_some_and(may_start_a_name);
        (starts_well && name.chars().all(may_be_in_a_name))
            .then(|| name.to_owned())
            .map(Self)
    }

    delegate::delegate! {
        to self.0 {
            /// The name, as a snippet's file gives it.
            pub fn as_str(&self) -> &str;
        }
    }

    pub(crate) fn crate_name(&self) -> String {
        format!("snippet_{}", self.0.replace('-', "_"))
    }
}

impl Borrow<str> for SnippetName {
    fn borrow(&self) -> &str {
        &self.0
    }
}

fn may_start_a_name(first: char) -> bool {
    may_be_in_a_name(first) && !first.is_ascii_digit()
}

fn may_be_in_a_name(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_' || character == '-'
}

#[cfg(test)]
mod tests;
