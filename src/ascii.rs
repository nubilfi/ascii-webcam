//! # ASCII Conversion
//!
//! This module provides functionality for converting video frames
//! to ASCII art representations.

use crate::error::Result;
use color_eyre::eyre::WrapErr;
use lazy_static::lazy_static;
use opencv::{
    core::{Mat, Size},
    imgproc,
    prelude::*,
};

lazy_static! {
    static ref ASCII_CHARS: Vec<char> = vec![' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
}

/// Converts a grayscale value to an ASCII character.
///
/// # Arguments
///
/// * `value` - The grayscale value to convert (0-255)
///
/// # Returns
///
/// An ASCII character from the `ASCII_CHARS` array, chosen based on the intensity of the
/// grayscale value. The mapping is such that `0` corresponds to the lightest character (`' '`),
/// and `255` corresponds to the darkest character (`'@'`).
///
/// # Examples
///
/// ```
/// use ascii_webcam::ascii::get_ascii_char;
/// assert_eq!(get_ascii_char(0), ' ');
/// ```
pub fn get_ascii_char(value: u8) -> char {
    let index = (value as usize * (ASCII_CHARS.len() - 1)) / 255;
    ASCII_CHARS[index.min(ASCII_CHARS.len() - 1)]
}

/// Processes a video frame and converts it to an ASCII art representation.
///
/// # Arguments
///
/// * `frame` - The video frame to process
/// * `width` - The width to resize the frame to
/// * `height` - The height to resize the frame to
///
/// # Returns
///
/// Returns a `Result<String>`. On success, it returns a string containing the ASCII art representation
/// of the resized and grayscale-converted video frame. Each line of the string represents a row
/// of ASCII characters corresponding to the pixels in the resized frame. On failure, it returns
/// an error with a descriptive message.
pub fn process_frame(frame: &Mat, width: i32, height: i32) -> Result<String> {
    let mut gray = Mat::default();
    imgproc::cvt_color(frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)
        .wrap_err("failed to convert frame to grayscale")?;

    let mut resized = Mat::default();
    imgproc::resize(
        &gray,
        &mut resized,
        Size::new(width, height),
        0.0,
        0.0,
        imgproc::INTER_LINEAR,
    )
    .wrap_err("failed to resize frame")?;

    let (rows, cols) = (resized.rows(), resized.cols());

    let ascii_frame = (0..rows)
        .map(|y| {
            (0..cols)
                .map(|x| {
                    let pixel = resized
                        .at_2d::<u8>(y, x)
                        .wrap_err("failed to access pixel")?;
                    Ok(get_ascii_char(*pixel))
                })
                .collect::<Result<String>>()
        })
        .collect::<Result<Vec<String>>>()?
        .join("\n");

    Ok(ascii_frame)
}
