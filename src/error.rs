//! # Error Handling
//!
//! This module defines custom error types and a Result type alias
//! for use throughout the ASCII Webcam application.

use color_eyre::Report;
use thiserror::Error;

/// A type alias for Results that use our custom AppError type.
pub type Result<T> = std::result::Result<T, Report>;

/// Custom error types for the ASCII Webcam application.
#[derive(Error, Debug)]
pub enum AppError {
    /// Represents I/O errors.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Represents OpenCV-related errors.
    #[error("OpenCV error: {0}")]
    OpenCV(#[from] opencv::Error),

    /// Represents errors related to terminal operations.
    #[error("Terminal error: {0}")]
    Terminal(String),

    /// Represents errors related to camera operations.
    #[error("Camera error: {0}")]
    Camera(String),

    /// Represents unknown errors
    #[allow(dead_code)]
    #[error("Unexpected error occurred: {0}")]
    Other(String),
}
