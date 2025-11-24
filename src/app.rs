//! # Application State and Rendering
//!
//! This module contains the `App` struct which represents the application state
//! and provides methods for updating and rendering the application.

use crate::ascii::process_frame;
use crate::error::Result;
use color_eyre::eyre::WrapErr;
use opencv::core::Mat;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

/// Represents the state of the application.
#[derive(Default)]
pub struct App {
    pub ascii_frame: String,
    pub fps: f64,
    pub show_help: bool,
    pub show_fps: bool,
}

impl App {
    /// Creates a new `App` instance with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use ascii_webcam::app::App;
    /// let app = App::new();
    /// assert_eq!(app.ascii_frame, "");
    /// assert_eq!(app.fps, 0.0);
    /// assert_eq!(app.show_help, false);
    /// ```
    #[must_use]
    pub fn new() -> App {
        App {
            ascii_frame: String::new(),
            fps: 0.0,
            show_help: false,
            show_fps: false,
        }
    }

    /// Updates the application state with a new video frame.
    ///
    /// # Arguments
    ///
    /// * `frame` - The video frame to process
    /// * `width` - The width to resize the frame to
    /// * `height` - The height to resize the frame to
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the update was successful.
    ///
    /// # Errors
    ///
    /// This function may return an error if:
    /// - The frame processing fails
    /// - There are issues with resizing or converting the frame
    pub fn update(&mut self, frame: &Mat, width: i32, height: i32) -> Result<()> {
        self.ascii_frame =
            process_frame(frame, width, height).wrap_err("failed to process frame")?;
        Ok(())
    }

    /// Toggles the visibility of the help menu.
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    /// Toggles the visibility of the fps.
    pub fn toggle_fps(&mut self) {
        self.show_fps = !self.show_fps;
    }

    /// Renders the application UI.
    ///
    /// This method is responsible for rendering:
    /// - The FPS counter
    /// - The ASCII video frame
    /// - The instruction text
    /// - The help menu (if visible)
    pub fn render(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(f.area());

        let instructions = Line::from(vec!["Help".into(), " <?>".cyan().bold()]);
        let ascii_block = Block::default()
            .borders(Borders::ALL)
            .title("ASCII Webcam")
            .title_bottom(instructions.alignment(ratatui::layout::Alignment::Center));
        let ascii_paragraph = Paragraph::new(self.ascii_frame.as_str()).block(ascii_block);

        f.render_widget(ascii_paragraph, chunks[1]);

        if self.show_help {
            self.render_help(f);
        }

        if self.show_fps {
            self.render_fps(f);
        }
    }

    /// Renders the help menu.
    #[allow(clippy::unused_self)]
    fn render_help(&self, f: &mut Frame) {
        let area = f.area();
        let help_area = Rect::new(
            area.width / 4,
            area.height / 4,
            area.width / 2,
            area.height / 2,
        );

        f.render_widget(Clear, help_area);

        let help_text = vec![
            Line::from("Help"),
            Line::from(""),
            Line::from(vec![
                Span::raw("Press "),
                Span::styled(
                    "q",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                ),
                Span::raw(" to quit the application"),
            ]),
            Line::from(vec![
                Span::raw("Press "),
                Span::styled(
                    "f",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                ),
                Span::raw(" to show FPS counter"),
            ]),
            Line::from(vec![
                Span::raw("Press "),
                Span::styled(
                    "?",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                ),
                Span::raw(" to toggle this help menu"),
            ]),
        ];

        let help_paragraph = Paragraph::new(help_text)
            .block(Block::default().title("Help").borders(Borders::ALL))
            .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(help_paragraph, help_area);
    }

    /// Renders the FPS overlay.
    fn render_fps(&self, f: &mut Frame) {
        let area = f.area();
        // Position in top-right corner
        let fps_area = Rect::new(
            area.width.saturating_sub(20), // 20 chars wide, positioned at right
            1,                             // 1 row from top
            19,                            // width
            3,                             // height for border + text
        );

        f.render_widget(Clear, fps_area);

        let fps_text = format!("FPS: {:.0}", self.fps);
        let fps_paragraph = Paragraph::new(fps_text)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL))
            .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(fps_paragraph, fps_area);
    }
}
