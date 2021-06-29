use std::{io, result};

use thiserror::Error;

/// A type that represents a success or failure.
pub type Result<T> = result::Result<T, Error>;

/// An enum that captures all possible error conditions of this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// An out of bounds index was accessed in a data structure.
    #[error("out of bounds index {index} exceeds length {length} in {name}")]
    OutOfBounds {
        index: usize,
        length: usize,
        name: String,
    },

    /// An error occurred while rendering a shape.
    #[error("render error: {0}")]
    Render(#[from] cairo::Error),

    /// An error occurred while cairo was doing I/O.
    #[error("cairo I/O error")]
    CairoIO(#[from] cairo::IoError),

    /// An I/O error occurred.
    #[error("file I/O error")]
    FileIO(#[from] io::Error),

    /// User-provided shape parameters were invalid.
    #[error("invalid shape parameters")]
    InvalidShape,

    /// User-provided color parameters were invalid.
    #[error("invalid color parameters")]
    InvalidColor,
}
