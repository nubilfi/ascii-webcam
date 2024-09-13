pub mod app;
pub mod ascii;
pub mod error;
pub mod terminal;
pub mod video;

// Re-export key types for convenience
pub use app::App;
pub use error::{AppError, Result};
pub use video::VideoCapture;
