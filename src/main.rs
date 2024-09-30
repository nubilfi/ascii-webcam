//! # ASCII Webcam Application
//!
//! This is the main entry point for the ASCII Webcam application.
//! It sets up the terminal, initializes the camera, and runs the main application loop.

use crate::app::App;
use crate::error::Result;
use crate::terminal::{reset_terminal, setup_terminal};
use crate::video::VideoCapture;
use color_eyre::eyre::WrapErr;
use crossbeam_channel::{bounded, select};
use crossterm::event::{self, Event, KeyCode};
use ratatui::Terminal;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

mod app;
mod ascii;
mod error;
mod terminal;
mod video;

/// Target frames per second for the application
const TARGET_FPS: u64 = 30;
/// Size of the circular buffer used for FPS calculation
const FPS_BUFFER_SIZE: usize = 120;

/// The main function of the application.
///
/// It performs the following steps:
/// 1. Installs `color_eyre` for error handling
/// 2. Sets up the terminal
/// 3. Initializes the camera
/// 4. Runs the main application loop
/// 5. Resets the terminal before exiting
fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = setup_terminal().wrap_err("failed to setup terminal")?;
    let camera = Arc::new(Mutex::new(
        VideoCapture::new(0).wrap_err("failed to initialize camera")?,
    ));
    let mut app = App::new();

    let res = run_app(&mut terminal, &mut app, camera);

    reset_terminal().wrap_err("failed to reset terminal")?;
    res
}

/// Runs the main application loop.
///
/// This function is responsible for:
/// - Setting up multi-threaded frame capture and event handling
/// - Updating the application state
/// - Rendering frames
/// - Handling user input
/// - Maintaining the target frame rate
/// - Calculating a stable FPS using a circular buffer
fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    camera: Arc<Mutex<VideoCapture>>,
) -> Result<()> {
    // Set up channels for communication between threads
    let (frame_sender, frame_receiver) = bounded(2);
    let (event_sender, event_receiver) = bounded(10);

    // Spawn frame capture thread
    let camera_clone = Arc::clone(&camera);
    thread::spawn(move || loop {
        if let Ok(mut camera) = camera_clone.lock() {
            if let Ok(frame) = camera.read_frame() {
                if frame_sender.send(frame).is_err() {
                    break;
                }
            }
        }
    });

    // Spawn event handling thread
    thread::spawn(move || loop {
        if event::poll(Duration::from_millis(1)).unwrap() {
            if let Ok(event) = event::read() {
                if event_sender.send(event).is_err() {
                    break;
                }
            }
        }
    });

    // Initialize FPS calculation buffer
    let mut fps_buffer = vec![Duration::from_secs(1); FPS_BUFFER_SIZE];
    let mut fps_index = 0;

    let target_frame_time = Duration::from_micros(1_000_000 / TARGET_FPS);

    loop {
        let frame_start = Instant::now();

        // Use select! macro to handle both frame processing and events
        select! {
            recv(frame_receiver) -> frame => {
                if let Ok(frame) = frame {
                    let size = terminal.size().wrap_err("failed to get terminal size")?;
                    let term_width = i32::from(size.width);
                    let term_height = i32::from(size.height);
                    app.update(&frame, term_width, term_height)
                        .wrap_err("failed to update app state")?;

                    terminal
                        .draw(|f| app.render(f))
                        .wrap_err("failed to render frame")?;

                    // Update FPS calculation
                    let frame_time = frame_start.elapsed();
                    fps_buffer[fps_index] = frame_time;
                    fps_index = (fps_index + 1) % FPS_BUFFER_SIZE;

                    let avg_frame_time = fps_buffer.iter().sum::<Duration>() / FPS_BUFFER_SIZE as u32;
                    app.fps = 1.0 / avg_frame_time.as_secs_f64();
                }
            }
            recv(event_receiver) -> event => {
                if let Ok(Event::Key(key)) = event {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('?') => app.toggle_help(),
                        _ => {}
                    }
                }
            }
        }

        // Maintain target frame rate
        let processing_time = frame_start.elapsed();
        if processing_time < target_frame_time {
            thread::sleep(target_frame_time - processing_time);
        }
    }
}
