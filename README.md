# ASCII Webcam

[![version](https://img.shields.io/crates/v/ascii-webcam?color=blue&logo=rust&style=flat-square)](https://crates.io/crates/ascii-webcam)
[![Build Status](https://github.com/nubilfi/ascii-webcam/actions/workflows/rust.yml/badge.svg)](https://github.com/nubilfi/ascii-webcam/actions?branch=main)
[![Documentation](https://docs.rs/ascii-webcam/badge.svg)](https://docs.rs/ascii-webcam/latest/ascii-webcam/)
[![codecov](https://codecov.io/gh/nubilfi/ascii-webcam/graph/badge.svg?token=SRGOFSB31Q)](https://codecov.io/gh/nubilfi/ascii-webcam)

ASCII Webcam is an application that captures video from your webcam and displays it as ASCII art in real-time within your terminal. This project demonstrates the use of OpenCV for video capture, image processing techniques, and terminal-based UI rendering using the [Ratatui](https://ratatui.rs/) library.

## Features

- Real-time webcam capture and ASCII conversion
- Terminal-based user interface with Ratatui
- FPS counter
- Resizable ASCII output adapting to terminal dimensions
- Help menu

## Requirements

- OpenCV 4.x
- A compatible webcam

## Installation

1. Install OpenCV 4.x. The installation process varies depending on your operating system:
   - On Archlinux: `sudo pacman -Sy opencv`
   - For other systems, please refer to the [OpenCV installation guide](https://docs.opencv.org/4.x/d7/d9f/tutorial_linux_install.html).

2. Clone the repository:
   ```
   git clone https://github.com/nubilfi/ascii-webcam.git
   cd ascii-webcam
   ```

3. Build the project:
   ```
   cargo b -r
   ```

4. Run test:
   ```
   cargo t
   ```

5. Run benchmark:
   ```
   cargo bench
   ```

## Usage

Run the application with:

```
cargo r
```

Once the application starts:

- The main window displays the ASCII representation of your webcam feed.
- The top bar shows the current FPS.
- Press `?` to toggle the help menu.
- Press `q` to quit the application.

## Project Structure

- `main.rs`: Entry point of the application, sets up the terminal and runs the main loop.
- `app.rs`: Defines the `App` struct and methods for updating and rendering the application state.
- `ascii.rs`: Contains functions for converting video frames to ASCII art.
- `error.rs`: Defines custom error types and result aliases.
- `terminal.rs`: Provides functions for setting up and resetting the terminal.
- `video.rs`: Implements a wrapper around OpenCV's VideoCapture for easy integration.

## License

[MIT](https://github.com/nubilfi/ascii-webcam/blob/main/LICENSE)

## Acknowledgments

- [OpenCV](https://opencv.org/) for computer vision capabilities.
- [Ratatui](https://github.com/ratatui-org/ratatui) for the terminal user interface.

## Notes

Hey there! Just wanted to let you know that this ASCII Webcam project is primarily a fun learning exercise. It's been a blast to work on, and I've learned many things about Rust, OpenCV, and terminal UIs in the process.

While this is a personal learning project, I'm always open to feedback, suggestions, or even pull requests if you're feeling adventurous!

Feel free to experiment with the code, break things (and then fix them!), or use it as a starting point for your own cool projects. If you come up with any interesting modifications or improvements, I'd love to see them!

Contributions are welcome! 
