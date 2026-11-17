//! Entity: the Rust edition every snippet is compiled with.

use derive_more::Display;

/// The Rust edition every snippet of a compilation is compiled with.
///
/// Displayed as a manifest writes it, such as `2021`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[non_exhaustive]
pub enum Edition {
    /// The 2015 edition.
    #[display("2015")]
    E2015,
    /// The 2018 edition.
    #[display("2018")]
    E2018,
    /// The 2021 edition.
    #[display("2021")]
    E2021,
    /// The 2024 edition.
    #[display("2024")]
    E2024,
}

#[cfg(test)]
mod tests;
