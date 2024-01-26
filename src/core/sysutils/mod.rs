//! # System specific utilities
//!
//! Some things works differently on different systems, so this module will abstract them away to provide a unified interface to do the same thing on different systems.

use std::{
    fmt::{self, Display, Formatter},
    io, result,
};

#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::*;

#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use unix::*;

/// Groups all the errors that can happen in this module independently of the system.
#[derive(Debug)]
pub enum Error {
    /// An error that happened while doing an IO operation.
    Io(io::Error),

    /// An error that happened while converting a [std::path::PathBuf] to a [str].
    CantConvertPathToStr,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "Io error: {}", e),
            Error::CantConvertPathToStr => write!(f, "Can't convert path to str"),
        }
    }
}

/// A specialized [result::Result] type for this module.
pub type Result<T> = result::Result<T, Error>;
