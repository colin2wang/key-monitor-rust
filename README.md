# Key Monitor Rust

A desktop application built with Rust and egui that monitors keyboard input and displays logs in real-time.

## Features

- **Real-time Log Display**: View application logs with color-coded severity levels
- **Log Level Filtering**: Filter logs by severity (Error, Warn, Info, Debug, Trace)
- **Auto-scroll**: Automatically scrolls to show the latest logs
- **Keyboard Input Prevention**: Periodically presses NumLock key to prevent system sleep
- **Cross-platform UI**: Built with egui/eframe for a native look and feel

## Project Structure

```
├── build.ps1           # Build script
├── build.rs            # Cargo build script (icon embedding)
├── resources/
│   └── app_icon.ico    # Application icon
├── src/
│   ├── logger/
│   │   ├── mod.rs          # Logger module exports
│   │   ├── collector.rs    # Log collector implementation
│   │   └── view.rs         # Log view UI component
│   ├── keyboard_controller.rs  # Keyboard input controller
│   └── main.rs             # Application entry point
└── dist/               # Build output (generated)
```

## Prerequisites

- Rust toolchain (latest stable version)
- Windows operating system (for keyboard input functionality)

## Building

```bash
# Clone the repository
git clone <repository-url>
cd key-monitor-rust

# Release build (default) - outputs to dist/
.\build.ps1

# Debug build
.\build.ps1 -Mode debug

# Clean build artifacts
.\build.ps1 -Clean

# Or use cargo directly
cargo build --release
```

The release build uses LTO, single codegen unit, size optimization (`opt-level = "z"`), symbol stripping, and abort-on-panic to minimize binary size.

## Running

```bash
# Run directly
cargo run --release

# Or run from dist/
.\dist\key-monitor-rust.exe
```

## Usage

### Log Controls

- **Clear Logs**: Clear all displayed logs
- **Start/Stop Key Press**: Begin or stop periodic NumLock key presses (green text = ready to start, red text = currently running)
- **Log Level Dropdown**: Select the minimum log level to display

### Log Levels

- **Error**: Critical errors only
- **Warn**: Warnings and errors
- **Info**: General information, warnings, and errors (default)
- **Debug**: Debug messages and above
- **Trace**: All messages including detailed trace information

## Technical Details

### Dependencies

- **egui**: Immediate mode GUI library
- **eframe**: egui framework for native applications
- **log**: Logging facade
- **env_logger**: Logger implementation
- **chrono**: Date and time handling
- **winapi**: Windows API bindings (Windows only)

### Architecture

The application uses a modular architecture:

1. **Logger Module**: Handles log collection and display
   - `collector.rs`: Captures log output and stores it in a shared buffer
   - `view.rs`: Renders logs in the UI with filtering and color-coding

2. **Keyboard Controller**: Manages keyboard input thread
   - Prevents system sleep by periodically pressing NumLock
   - Runs in a separate thread with controlled start/stop

3. **Main Application**: Orchestrates UI and business logic
   - Creates and manages the application window
   - Handles user interactions

## Development

### Code Style

This project follows standard Rust conventions:
- Snake_case for functions and variables
- PascalCase for types and structs
- SCREAMING_SNAKE_CASE for constants
- Comprehensive documentation comments

### Adding New Features

1. Create new modules in appropriate directories
2. Update `main.rs` to include new modules
3. Add necessary dependencies to `Cargo.toml`
4. Follow existing code patterns and style

## License

This project is open source. See the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues.

## Version

Current version: 0.1.2
