# rust-system-monitor

A terminal-based system monitor written in **Rust** using `ratatui`,
`crossterm`, and `sysinfo`.

This project provides a simple **TUI (Terminal User Interface)**
dashboard to monitor system resources such as:

-   CPU usage
-   Memory usage
-   Disk usage
-   Top running processes

The application updates in real time and is designed as a learning
project for building **interactive terminal applications in Rust**.

------------------------------------------------------------------------

# Features

-   Real-time system monitoring
-   CPU usage overview
-   Memory and swap usage
-   Disk usage by mount point
-   Top processes sorted by memory usage
-   Interactive terminal interface (TUI)
-   Keyboard shortcuts for navigation

------------------------------------------------------------------------

# Demo

    +--------------------------------------------------------+
    | Server Monitor                                         |
    | CPU: 23% | Cores: 8 | Load: 0.41 0.55 0.62             |
    | RAM: 6.20 / 15.90 GB (39%)                             |
    +--------------------+-----------------------------------+
    | Disks              | Top Processes                     |
    | /      120GB/256GB | PID   MEM(MB)  CPU%   NAME        |
    | /data   50GB/200GB | 1203  230.5    2.3    chrome      |
    |                    | 998   180.2    1.1    code        |
    |                    | 411   120.4    0.8    docker      |
    +--------------------------------------------------------+
    | Keys: q = quit | +/- change process count              |
    +--------------------------------------------------------+

------------------------------------------------------------------------

# Tech Stack

This project is built with the following Rust libraries:

-   **ratatui** -- terminal UI framework
-   **crossterm** -- cross-platform terminal handling
-   **sysinfo** -- system information (CPU, memory, processes, disks)

------------------------------------------------------------------------

# Project Structure

    src/
    ├── main.rs      # Application entry point
    ├── app.rs       # Application state and keyboard handling
    ├── sys.rs       # System information reader (CPU, RAM, disks, processes)
    ├── ui.rs        # Terminal UI rendering using ratatui
    └── event.rs     # Event loop (keyboard input + refresh tick)

### main.rs

Initializes the terminal, starts the event loop, and handles rendering.

### app.rs

Manages application state and user input (keyboard commands).

### sys.rs

Collects system metrics using the `sysinfo` crate.

### ui.rs

Responsible for rendering the TUI layout and widgets.

### event.rs

Handles terminal events and refresh timing.

------------------------------------------------------------------------

# Installation

## Requirements

-   Rust (latest stable)

Install Rust if you don't have it:

https://rustup.rs

------------------------------------------------------------------------

# Run the Application

Clone the repository:

    git clone https://github.com/IzePhanthakarn/rust-system-monitor.git
    cd rust-system-monitor

Run the application:

    cargo run

------------------------------------------------------------------------

# Controls

  Key     Action
  ------- ----------------------------------------
  `q`     Quit application
  `Esc`   Quit application
  `+`     Increase number of displayed processes
  `-`     Decrease number of displayed processes

------------------------------------------------------------------------

# Learning Goals

This project was created to explore:

-   Building **terminal UI applications in Rust**
-   Handling **real-time event loops**
-   Working with **system-level information**
-   Designing **modular Rust project structures**

------------------------------------------------------------------------

# Future Improvements

Possible enhancements for the project:

-   CPU usage per core
-   Memory usage charts
-   Network bandwidth monitoring
-   Process sorting and filtering
-   Scrollable process list
-   Theme and color customization

------------------------------------------------------------------------

# License

MIT License
