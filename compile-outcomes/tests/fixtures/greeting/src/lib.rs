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

/// Written so that clippy's `let_and_return` would fire on it, were a dependency linted.
pub fn answer() -> u32 {
    let answer = 42;
    answer
}
