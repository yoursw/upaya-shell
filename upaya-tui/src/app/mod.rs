//! Application state management

use upaya_core::{Result, PluginMetadata, UpayaPlugin};
use std::sync::Arc;

/// Available tabs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Plugins,
    Settings,
}

/// Application state
pub struct App {
    pub plugins: Vec<(PluginMetadata, Arc<dyn UpayaPlugin>)>,
    pub selected_plugin: Option<usize>,
    pub current_tab: Tab,
    pub plugin_output: String,
    pub debug_output: Vec<String>,
}

impl App {
    /// Create a new TUI application
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            selected_plugin: None,
            current_tab: Tab::Plugins,
            plugin_output: String::new(),
            debug_output: Vec::new(),
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
}
