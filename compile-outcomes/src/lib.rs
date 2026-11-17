//! Facade: compiles Rust snippets and reports what the compiler said about each one, without judging.

#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the service, which runs the policies over the model, is not written yet"
    )
)]
mod domain;
#[cfg(test)]
mod test_support;
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the service, which runs the policies, is not written yet"
    )
)]
mod use_cases;

pub use domain::{
    Compilation, Dependency, Diagnostic, Edition, Error, Errors, Location, Outcome, Outcomes,
    SnippetName,
};
