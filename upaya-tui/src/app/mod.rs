//! Application state management

use upaya_core::{Result, PluginMetadata, UpayaPlugin};
use std::sync::Arc;

/// Available tabs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Plugins,
    Load,
    Settings,
}

/// Menu items
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub title: String,
    pub description: String,
    pub action: MenuAction,
}

/// Menu actions
#[derive(Debug, Clone)]
pub enum MenuAction {
    LoadGit(String),
    LoadHttp(String),
    LoadLocal(String),
    CreateNew,
    Settings,
}

/// Application state
pub struct App {
    pub plugins: Vec<(PluginMetadata, Arc<dyn UpayaPlugin>)>,
    pub selected_plugin: Option<usize>,
    pub current_tab: Tab,
    pub plugin_output: String,
    pub debug_output: Vec<String>,
    pub menu_items: Vec<MenuItem>,
    pub selected_menu: Option<usize>,
    pub input: String,
    pub messages: Vec<String>,
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
            menu_items: Vec::new(),
            selected_menu: None,
            input: String::new(),
            messages: Vec::new(),
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

    /// Add a message
    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
        if self.messages.len() > 100 {
            self.messages.remove(0);
        }
    }

    /// Handle menu selection
    pub fn handle_menu_action(&mut self) {
        if let Some(idx) = self.selected_menu {
            if let Some(item) = self.menu_items.get(idx) {
                match &item.action {
                    MenuAction::LoadGit(_) => {
                        self.current_tab = Tab::Load;
                        self.input = "git://".to_string();
                    }
                    MenuAction::LoadHttp(_) => {
                        self.current_tab = Tab::Load;
                        self.input = "http://".to_string();
                    }
                    MenuAction::LoadLocal(_) => {
                        self.current_tab = Tab::Load;
                        self.input = "file://".to_string();
                    }
                    MenuAction::CreateNew => {
                        self.add_message("Create new plugin functionality not implemented yet".to_string());
                    }
                    MenuAction::Settings => {
                        self.current_tab = Tab::Settings;
                    }
                }
            }
        }
    }
} 