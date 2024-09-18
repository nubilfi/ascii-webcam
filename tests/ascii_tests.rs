// tests/ascii_tests.rs
use ascii_webcam::ascii::{get_ascii_char, process_frame};
use opencv::{core, imgproc};

#[test]
fn test_process_frame() {
    // Create a simple test image
    let mut frame =
        core::Mat::new_rows_cols_with_default(480, 640, core::CV_8UC3, core::Scalar::all(255.0))
            .unwrap();

    // Draw a rectangle on the image
    let _ = imgproc::rectangle(
        &mut frame,
        core::Rect::new(200, 100, 240, 280),
        core::Scalar::new(0.0, 0.0, 0.0, 0.0),
        -1,
        imgproc::LINE_8,
        0,
    );

    let result = process_frame(&frame, 80, 24);
    assert!(result.is_ok(), "Process frame failed: {:?}", result.err());
    let ascii_frame = result.unwrap();
    assert!(!ascii_frame.is_empty());

    // Additional checks
    assert!(
        ascii_frame.contains(' '),
        "ASCII frame should contain spaces for white areas"
    );
    assert!(
        ascii_frame.contains('@') || ascii_frame.contains('#'),
        "ASCII frame should contain dark characters for the rectangle"
    );
}

#[test]
fn test_get_ascii_char() {
    assert_eq!(get_ascii_char(0), ' ');
}

