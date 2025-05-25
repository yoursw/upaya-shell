//! Application state management

use upaya_core::{Result, PluginMetadata};

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
    pub plugins: Vec<PluginMetadata>,
    pub selected_plugin: Option<usize>,
    pub input: String,
    pub messages: Vec<String>,
    pub current_tab: Tab,
    pub menu_items: Vec<MenuItem>,
    pub selected_menu: Option<usize>,
}

impl App {
    /// Create a new TUI application
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            selected_plugin: None,
            input: String::new(),
            messages: Vec::new(),
            current_tab: Tab::Plugins,
            menu_items: vec![
                MenuItem {
                    title: "Load from Git".to_string(),
                    description: "Load a plugin from a Git repository".to_string(),
                    action: MenuAction::LoadGit(String::new()),
                },
                MenuItem {
                    title: "Load from HTTP".to_string(),
                    description: "Load a plugin from an HTTP URL".to_string(),
                    action: MenuAction::LoadHttp(String::new()),
                },
                MenuItem {
                    title: "Load from Local".to_string(),
                    description: "Load a plugin from local filesystem".to_string(),
                    action: MenuAction::LoadLocal(String::new()),
                },
                MenuItem {
                    title: "Create New Plugin".to_string(),
                    description: "Create a new plugin from template".to_string(),
                    action: MenuAction::CreateNew,
                },
                MenuItem {
                    title: "Settings".to_string(),
                    description: "Configure Upaya Shell".to_string(),
                    action: MenuAction::Settings,
                },
            ],
            selected_menu: None,
        }
    }

    /// Update plugin list
    pub fn update_plugins(&mut self, plugins: Vec<PluginMetadata>) {
        self.plugins = plugins;
    }

    /// Add a message to the message log
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