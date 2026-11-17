//! Facade: compiles Rust snippets and reports what the compiler said about each one, without judging.

#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the policies and the service, which read the model, are not written yet"
    )
)]
mod domain;
#[cfg(test)]
mod test_support;

pub use domain::{
    Compilation, Dependency, Diagnostic, Edition, Error, Errors, Location, Outcome, Outcomes,
    SnippetName,
};
