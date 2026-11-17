//! Adapter: the code that does input and output, implementing the ports.

mod cargo;
mod disk;

pub(crate) use cargo::CargoProcess;
pub(crate) use disk::FileSystem;
