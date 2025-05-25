//! Example plugin for Upaya Shell
//!
//! This crate provides an example plugin implementation.

use std::sync::Mutex;
use upaya_core::{Result, UpayaPlugin, PluginMetadata, PluginPermissions};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crossterm::event::{Event, KeyCode, KeyEvent};

/// Example plugin implementation
pub struct ExamplePlugin {
    metadata: PluginMetadata,
    state: Mutex<PluginState>,
}

struct PluginState {
    input_buffer: String,
    input_cursor: usize,
    output: String,
}

impl ExamplePlugin {
    /// Create a new example plugin
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                name: "example".to_string(),
                version: "0.1.0".to_string(),
                description: "An example plugin for Upaya Shell".to_string(),
                author: "Your SW <admin@kusala.tech>".to_string(),
                permissions: PluginPermissions {
                    filesystem: false,
                    network: false,
                    system: false,
                },
            },
            state: Mutex::new(PluginState {
                input_buffer: String::new(),
                input_cursor: 0,
                output: String::new(),
            }),
        }
    }
}

impl UpayaPlugin for ExamplePlugin {
    fn metadata(&self) -> PluginMetadata {
        self.metadata.clone()
    }

    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn execute(&self, args: &[String]) -> Result<String> {
        Ok(format!("Example plugin executed with args: {:?}", args))
    }

    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    fn render(&self, f: &mut Frame, area: Rect) -> Result<()> {
        let state = self.state.lock().unwrap();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Plugin info
                Constraint::Length(3),  // Input box
                Constraint::Min(0),     // Output
            ])
            .split(area);

        // Render plugin info
        let info = vec![
            Line::from(format!("Plugin: {}", self.metadata.name)),
            Line::from("Enter your input below:"),
        ];
        let info = Paragraph::new(info)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(info, chunks[0]);

        // Render input box
        let input = format!("{}", state.input_buffer);
        let input = Paragraph::new(input)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(input, chunks[1]);

        // Render output
        let output = if !state.output.is_empty() {
            state.output.as_str()
        } else {
            "No output yet"
        };
        let output = Paragraph::new(output)
            .block(Block::default().title("Output").borders(Borders::ALL));
        f.render_widget(output, chunks[2]);

        // Render cursor
        f.set_cursor(
            chunks[1].x + state.input_cursor as u16 + 1,
            chunks[1].y + 1,
        );

        Ok(())
    }

    fn handle_event(&self, event: &Event) -> Result<bool> {
        if let Event::Key(KeyEvent { code, .. }) = event {
            let mut state = self.state.lock().unwrap();
            match code {
                KeyCode::Left => {
                    if state.input_cursor > 0 {
                        state.input_cursor -= 1;
                    }
                    return Ok(true);
                }
                KeyCode::Right => {
                    if state.input_cursor < state.input_buffer.len() {
                        state.input_cursor += 1;
                    }
                    return Ok(true);
                }
                KeyCode::Backspace => {
                    if state.input_cursor > 0 {
                        let cursor_pos = state.input_cursor;
                        state.input_buffer.remove(cursor_pos - 1);
                        state.input_cursor -= 1;
                    }
                    return Ok(true);
                }
                KeyCode::Enter => {
                    state.output = format!("You entered: {}", state.input_buffer);
                    state.input_buffer.clear();
                    state.input_cursor = 0;
                    return Ok(true);
                }
                KeyCode::Char(c) => {
                    let cursor_pos = state.input_cursor;
                    state.input_buffer.insert(cursor_pos, *c);
                    state.input_cursor += 1;
                    return Ok(true);
                }
                _ => {}
            }
        }
        Ok(false)
    }
}
