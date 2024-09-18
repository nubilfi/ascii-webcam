use ascii_webcam::video::VideoCapture;
use opencv::core::MatTraitConst;

#[test]
#[ignore = "need to check you webcam during test"]
fn test_video_capture_creation() {
    let result = VideoCapture::new(0);
    assert!(result.is_ok());
}

#[test]
#[ignore = "need to check you webcam during test"]
fn test_read_frame() {
    let mut capture = VideoCapture::new(0).unwrap();
    let result = capture.read_frame();
    assert!(result.is_ok());
    let frame = result.unwrap();
    assert!(!frame.empty());
}
