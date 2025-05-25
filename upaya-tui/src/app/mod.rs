//! Application state management

use upaya_core::{Result, PluginMetadata, UpayaPlugin};
use std::sync::Arc;

/// Available tabs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Plugins,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PluginViewState {
    List,
    Details,
    Input,
    Output,
}

/// Application state
pub struct App {
    pub plugins: Vec<(PluginMetadata, Arc<dyn UpayaPlugin>)>,
    pub selected_plugin: Option<usize>,
    pub current_tab: Tab,
    pub plugin_view_state: PluginViewState,
    pub plugin_output: String,
    pub debug_output: Vec<String>,
    pub input_buffer: String,
    pub input_cursor: usize,
}

impl App {
    /// Create a new TUI application
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            selected_plugin: None,
            current_tab: Tab::Plugins,
            plugin_view_state: PluginViewState::List,
            plugin_output: String::new(),
            debug_output: Vec::new(),
            input_buffer: String::new(),
            input_cursor: 0,
        }
    }

    /// Update plugin list
    pub fn update_plugins(&mut self, plugins: Vec<(PluginMetadata, Arc<dyn UpayaPlugin>)>) {
        self.plugins = plugins;
    }

    /// Add debug output
    pub fn add_debug(&mut self, output: String) {
        self.debug_output.push(output);
        if self.debug_output.len() > 100 {
            self.debug_output.remove(0);
        }
    }

    /// Add plugin output
    pub fn add_plugin_output(&mut self, output: String) {
        self.plugin_output = output;
    }

    /// Clear plugin output
    pub fn clear_plugin_output(&mut self) {
        self.plugin_output.clear();
    }

    /// Add character to input buffer
    pub fn add_input_char(&mut self, c: char) {
        self.input_buffer.insert(self.input_cursor, c);
        self.input_cursor += 1;
    }

    /// Remove character from input buffer
    pub fn remove_input_char(&mut self) {
        if self.input_cursor > 0 {
            self.input_buffer.remove(self.input_cursor - 1);
            self.input_cursor -= 1;
        }
    }

    /// Clear input buffer
    pub fn clear_input(&mut self) {
        self.input_buffer.clear();
        self.input_cursor = 0;
    }
}
