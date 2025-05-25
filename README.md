# Upaya Shell

A modular TUI (Terminal User Interface) toolbox with plugin support, built in Rust.

## Features

- Modern TUI interface using `ratatui` and `crossterm`
- Plugin system supporting both native Rust and WebAssembly plugins
- Real-time plugin output display
- Debug console for development and troubleshooting
- Tab-based navigation between plugins and settings
- Plugin metadata and permission system

## Project Structure

```
upaya-shell/
├── upaya-core/      # Core functionality and plugin interfaces
├── upaya-tui/       # Main TUI application
├── upaya-plugin/    # Plugin development utilities
└── demo/            # Example plugin implementation
```

## Development

### Prerequisites

- Rust 1.75 or later
- Cargo (Rust's package manager)

### Building

```bash
# Build all crates
cargo build

# Run the demo application
cargo run -p upaya-demo
```

### Plugin Development

Plugins implement the `UpayaPlugin` trait from `upaya-core`. The trait provides methods for:

- Plugin metadata and permissions
- Initialization and cleanup
- UI rendering
- Event handling
- Command execution

Example plugin implementation can be found in the `demo` crate.

### Running the Demo

The demo application includes an example plugin that demonstrates:

- Text input handling
- Cursor navigation
- Output display
- Event processing

To run the demo:

```bash
cargo run -p upaya-demo
```

## License

AGPL-3.0

## Author

Your SW <admin@kusala.tech>
