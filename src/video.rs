//! # Video Capture
//!
//! This module provides a wrapper around `OpenCV`'s `VideoCapture`
//! for easy integration with the ASCII Webcam application.

use crate::error::{AppError, Result};
use color_eyre::eyre::WrapErr;
use opencv::{
    core::Mat,
    prelude::*,
    videoio::{VideoCapture as OpenCVVideoCapture, CAP_ANY},
};

/// A wrapper around `OpenCV`'s `VideoCapture`.
#[allow(clippy::module_name_repetitions)]
pub struct VideoCapture {
    capture: OpenCVVideoCapture,
}

impl VideoCapture {
    /// Creates a new `VideoCapture` instance.
    ///
    /// # Arguments
    ///
    /// * `camera_index` - The index of the camera to use
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ascii_webcam::video::VideoCapture;
    /// let capture = VideoCapture::new(0);
    /// assert!(capture.is_ok());
    /// ```
    /// # Errors
    ///
    /// This function may return an error if:
    /// - The video capture device cannot be initialized
    /// - The specified camera index is invalid or the camera is not accessible
    pub fn new(camera_index: i32) -> Result<Self> {
        let capture = OpenCVVideoCapture::new(camera_index, CAP_ANY)
            .wrap_err("failed to create VideoCapture")?;
        Ok(VideoCapture { capture })
    }

    /// Reads a frame from the video capture device.
    ///
    /// Returns an error if the frame is empty.
    ///
    /// # Errors
    ///
    /// This function may return an error if:
    /// - The frame cannot be read from the capture device
    /// - The captured frame is empty
    /// - There are issues with the camera or its connection
    pub fn read_frame(&mut self) -> Result<Mat> {
        let mut frame = Mat::default();
        self.capture
            .read(&mut frame)
            .wrap_err("failed to read frame")?;
        if frame.empty() {
            Err(AppError::Camera("empty frame".to_string()).into())
        } else {
            Ok(frame)
        }
    }
}
