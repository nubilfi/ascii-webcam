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
    pub fn new(camera_index: i32) -> Result<Self> {
        let capture = OpenCVVideoCapture::new(camera_index, CAP_ANY)
            .wrap_err("failed to create VideoCapture")?;
        Ok(VideoCapture { capture })
    }

    /// Reads a frame from the video capture device.
    ///
    /// Returns an error if the frame is empty.
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
