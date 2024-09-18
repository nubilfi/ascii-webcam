use ascii_webcam::terminal::{reset_terminal, setup_terminal};

#[test]
#[ignore = "failed in github action"]
fn test_terminal_setup_and_reset() {
    let terminal_result = setup_terminal();
    assert!(terminal_result.is_ok());

    let reset_result = reset_terminal();
    assert!(reset_result.is_ok());
}
