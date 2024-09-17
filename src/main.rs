//! # ASCII Webcam Application
//!
//! This is the main entry point for the ASCII Webcam application.
//! It sets up the terminal, initializes the camera, and runs the main application loop.

use crate::app::App;
use crate::error::Result;
use crate::terminal::{reset_terminal, setup_terminal};
use crate::video::VideoCapture;
use color_eyre::eyre::WrapErr;
use crossterm::event::{self, Event, KeyCode};
use ratatui::Terminal;
use std::time::{Duration, Instant};

mod app;
mod ascii;
mod error;
mod terminal;
mod video;

const TARGET_FPS: u64 = 30;

/// The main function of the application.
///
/// It performs the following steps:
/// 1. Installs `color_eyre` for error handling
/// 2. Sets up the terminal
/// 3. Initializes the camera
/// 4. Runs the main application loop
/// 5. Resets the terminal before exiting
fn main() -> Result<()> {
    // Install color_eyre
    color_eyre::install()?;

    let mut terminal = setup_terminal().wrap_err("failed to setup terminal")?;

    let mut camera = VideoCapture::new(0).wrap_err("failed to initialize camera")?;
    let mut app = App::new();

    let res = run_app(&mut terminal, &mut app, &mut camera);

    reset_terminal().wrap_err("failed to reset terminal")?;

    res
}

/// Runs the main application loop.
///
/// This function is responsible for:
/// - Updating the application state
/// - Rendering frames
/// - Handling user input
/// - Maintaining the target frame rate
fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    camera: &mut VideoCapture,
) -> Result<()> {
    let mut last_frame_time = Instant::now();
    let target_frame_time = Duration::from_micros(1_000_000 / TARGET_FPS);

    loop {
        let frame_start = Instant::now();

        let size = terminal.size().wrap_err("failed to get terminal size")?;
        let term_width = i32::from(size.width);
        let term_height = i32::from(size.height);

        if let Ok(frame) = camera.read_frame() {
            app.update(&frame, term_width, term_height)
                .wrap_err("failed to update app state")?;
        }

        terminal
            .draw(|f| app.render(f))
            .wrap_err("failed to render frame")?;

        if event::poll(Duration::from_millis(1)).wrap_err("failed to poll for events")? {
            if let Event::Key(key) = event::read().wrap_err("failed to read event")? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('?') => app.toggle_help(),
                    _ => {}
                }
            }
        }

        let current_frame_time = Instant::now();
        app.fps = 1.0
            / current_frame_time
                .duration_since(last_frame_time)
                .as_secs_f64();
        last_frame_time = current_frame_time;

        let processing_time = frame_start.elapsed();
        if processing_time < target_frame_time {
            std::thread::sleep(target_frame_time - processing_time);
        }
    }
}
