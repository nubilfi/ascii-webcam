# ASCII Webcam

[![version](https://img.shields.io/crates/v/ascii-webcam?color=blue&logo=rust&style=flat-square)](https://crates.io/crates/ascii-webcam)
[![Build Status](https://github.com/nubilfi/ascii-webcam/actions/workflows/rust.yml/badge.svg)](https://github.com/nubilfi/ascii-webcam/actions?branch=main)
[![Documentation](https://docs.rs/ascii-webcam/badge.svg)](https://docs.rs/ascii-webcam/latest/ascii_webcam/index.html)
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

2. Build the project:
   ```
   cargo b -r
   ```

3. Run test:
   ```
   cargo t
   ```

4. Run benchmark:
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

## License

[MIT](https://github.com/nubilfi/ascii-webcam/blob/main/LICENSE)

