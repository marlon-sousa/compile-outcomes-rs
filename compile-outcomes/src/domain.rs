//! Entity: what a compilation is made of, what it reports about each snippet, and what keeps it from reporting.

mod build_report;
mod compilation;
mod dependency;
mod diagnostic;
mod edition;
mod errors;
mod found;
mod outcomes;
mod project;
mod project_folder;
mod snippet;
mod snippet_name;

pub(crate) use build_report::{BuildReport, CompilerMessage, Level};
pub use compilation::Compilation;
pub(crate) use compilation::SnippetSource;
pub use dependency::Dependency;
pub(crate) use dependency::Source;
pub use diagnostic::{Diagnostic, Location};
pub use edition::Edition;
pub use errors::{Error, Errors};
pub(crate) use found::Found;
pub use outcomes::{Outcome, Outcomes};
pub(crate) use project::{Package, Project, Resolver};
pub(crate) use project_folder::{OWNER_MARKER, ProjectFolder};
pub(crate) use snippet::Snippet;
pub use snippet_name::SnippetName;
