//! Entity: what the disk holds where a compilation writes its project, and whose it is.

use strum::EnumIs;

/// The file that marks a project folder as compile-outcomes' own, which it writes into every project folder it makes.
///
/// A user who creates it in an existing folder hands that folder over, to be replaced.
pub(crate) const OWNER_MARKER: &str = ".owned-by-compile-outcomes";

/// What the disk holds at a project's root before the project is written.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIs)]
pub(crate) enum ProjectFolder {
    /// Nothing is there yet.
    Missing,
    /// An empty folder.
    Empty,
    /// A folder holding the owner marker.
    Owned,
    /// A folder holding files, without the owner marker.
    Foreign,
}
