//! A crate for the scenarios' snippets to depend on.

pub fn hello() -> &'static str {
    "hello"
}

#[cfg(feature = "polite")]
pub fn please() -> &'static str {
    "please"
}

#[cfg(feature = "loud")]
pub fn shout() -> &'static str {
    "HELLO"
}
