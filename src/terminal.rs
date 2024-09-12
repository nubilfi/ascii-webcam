//! # Terminal Setup and Management
//!
//! This module provides functions for setting up and resetting the terminal
//! for use with the ASCII Webcam application.

use crate::error::{AppError, Result};
use color_eyre::eyre::WrapErr;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, panic};

/// Sets up the terminal for the application.
///
/// This function:
/// 1. Sets up a panic hook to reset the terminal on panic
/// 2. Enables raw mode
/// 3. Enters the alternate screen
/// 4. Enables mouse capture
/// 5. Creates and returns a new Terminal instance
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = reset_terminal();
        original_hook(panic_info);
    }));

    enable_raw_mode().wrap_err("failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .wrap_err("failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);

    // Terminal::new(backend).wrap_err("Failed to create terminal")
    Terminal::new(backend).map_err(|e| AppError::Terminal(e.to_string()).into())
}

/// Resets the terminal to its original state.
///
/// This function:
/// 1. Disables raw mode
/// 2. Leaves the alternate screen
/// 3. Disables mouse capture
pub fn reset_terminal() -> Result<()> {
    disable_raw_mode().wrap_err("failed to disable raw mode")?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)
        .wrap_err("failed to leave alternate screen")?;
    Ok(())
}
