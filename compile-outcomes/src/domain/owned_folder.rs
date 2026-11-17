//! Entity: what the disk holds where a compilation replaces or removes a folder, and whose it is.

use strum::EnumIs;

/// The file that marks a project folder as compile-outcomes' own, which it writes into every project folder it makes.
///
/// A user who creates it in an existing folder hands that folder over, to be replaced.
pub(crate) const OWNER_MARKER: &str = ".owned-by-compile-outcomes";

/// The file Cargo writes into a target folder when it makes it, marking it as build output, as Cargo does from 1.85 to 1.97 at least.
pub(crate) const CARGO_MARKER: &str = "CACHEDIR.TAG";

/// What the disk holds at a folder compile-outcomes would replace or remove.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIs)]
pub(crate) enum OwnedFolder {
    /// Nothing is there yet.
    Missing,
    /// A folder holding no file at any depth: empty, or holding only empty folders, as one a CI cache stripped of its files.
    Empty,
    /// A folder holding the marker of whoever wrote it.
    Owned,
    /// A folder holding a file, without that marker.
    Foreign,
}
