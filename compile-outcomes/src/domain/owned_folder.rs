//! Entity: what the disk holds where a compilation writes, and whose it is.

use strum::EnumIs;

/// The file that marks a project folder as compile-outcomes' own, which it writes into every project folder it makes.
///
/// A user who creates it in an existing folder hands that folder over, to be replaced.
pub(crate) const OWNER_MARKER: &str = ".owned-by-compile-outcomes";

/// The file Cargo writes into every target folder it makes, marking it as build output.
pub(crate) const CARGO_MARKER: &str = "CACHEDIR.TAG";

/// What the disk holds at a folder compile-outcomes would replace or remove.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIs)]
pub(crate) enum OwnedFolder {
    /// Nothing is there yet.
    Missing,
    /// An empty folder.
    Empty,
    /// A folder holding the marker of whoever wrote it.
    Owned,
    /// A folder holding files, without that marker.
    Foreign,
}
