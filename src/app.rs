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
    pub fn new() -> App {
        App {
            ascii_frame: String::new(),
            fps: 0.0,
            show_help: false,
        }
    }

    /// Updates the application state with a new video frame.
    ///
    /// # Arguments
    ///
    /// * `frame` - The video frame to process
    /// * `width` - The width to resize the frame to
    /// * `height` - The height to resize the frame to
    pub fn update(&mut self, frame: &Mat, width: i32, height: i32) -> Result<()> {
        self.ascii_frame =
            process_frame(frame, width, height).wrap_err("failed to process frame")?;
        Ok(())
    }

    /// Toggles the visibility of the help menu.
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
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
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(f.area());

        let fps_text = format!("FPS: {:.2}", self.fps);
        let fps_paragraph = Paragraph::new(fps_text)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL).title("Stats"));

        f.render_widget(fps_paragraph, chunks[0]);

        let ascii_block = Block::default().borders(Borders::ALL).title("ASCII Webcam");
        let ascii_paragraph = Paragraph::new(self.ascii_frame.as_str()).block(ascii_block);

        f.render_widget(ascii_paragraph, chunks[1]);

        let instructions = Line::from(vec![
            "Quit".into(),
            " <q>".blue().bold(),
            " | Help".into(),
            " <?>".blue().bold(),
        ]);
        let instructions_paragraph = Paragraph::new(instructions)
            .style(Style::default().fg(Color::White))
            .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(instructions_paragraph, chunks[2]);

        if self.show_help {
            self.render_help(f);
        }
    }

    /// Renders the help menu.
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
                        .fg(Color::Blue)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                ),
                Span::raw(" to quit the application"),
            ]),
            Line::from(vec![
                Span::raw("Press "),
                Span::styled(
                    "?",
                    Style::default()
                        .fg(Color::Blue)
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
}
