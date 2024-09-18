use ascii_webcam::app::App;
use opencv::{core, imgproc};

#[test]
fn test_app_creation() {
    let app = App::new();
    assert_eq!(app.ascii_frame, "");
    assert_eq!(app.fps, 0.0);
    assert!(!app.show_help);
}

#[test]
fn test_app_update() {
    let mut app = App::new();

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

    let result = app.update(&frame, 80, 24);
    assert!(result.is_ok(), "App update failed: {:?}", result.err());
    assert!(!app.ascii_frame.is_empty());
}

#[test]
fn test_app_toggle_help() {
    let mut app = App::new();
    assert!(!app.show_help);
    app.toggle_help();
    assert!(app.show_help);
    app.toggle_help();
    assert!(!app.show_help);
}
