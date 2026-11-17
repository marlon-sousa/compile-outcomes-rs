//! Entity: what a compilation is made of, what it reports about each snippet, and what keeps it from reporting.

mod compilation;
mod dependency;
mod diagnostic;
mod edition;
mod errors;
mod outcomes;
mod snippet_name;

pub use compilation::Compilation;
pub use dependency::Dependency;
pub use diagnostic::{Diagnostic, Location};
pub use edition::Edition;
pub use errors::{Error, Errors};
pub use outcomes::{Outcome, Outcomes};
pub use snippet_name::SnippetName;
