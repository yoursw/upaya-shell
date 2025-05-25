//! Event handling

use crossterm::event::{Event, KeyCode};
use upaya_core::Result;
use crate::app::App;

/// Handle terminal events
pub fn handle_event(event: Event, app: &mut App) -> Result<()> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Char('q') => return Ok(()),
            KeyCode::Tab => {
                app.current_tab = match app.current_tab {
                    crate::app::Tab::Plugins => crate::app::Tab::Load,
                    crate::app::Tab::Load => crate::app::Tab::Settings,
                    crate::app::Tab::Settings => crate::app::Tab::Plugins,
                };
            }
            KeyCode::Char(c) => {
                if app.current_tab == crate::app::Tab::Load {
                    app.input.push(c);
                }
            }
            KeyCode::Backspace => {
                if app.current_tab == crate::app::Tab::Load {
                    app.input.pop();
                }
            }
            KeyCode::Enter => {
                match app.current_tab {
                    crate::app::Tab::Load => {
                        app.add_message(format!("Loading from: {}", app.input));
                        app.input.clear();
                    }
                    crate::app::Tab::Plugins => {
                        if let Some(idx) = app.selected_plugin {
                            if let Some(plugin) = app.plugins.get(idx) {
                                app.add_message(format!("Selected plugin: {}", plugin.name));
                            }
                        }
                    }
                    crate::app::Tab::Settings => {
                        app.add_message("Settings updated".to_string());
                    }
                }
            }
            KeyCode::Up => {
                match app.current_tab {
                    crate::app::Tab::Plugins => {
                        app.selected_plugin = app.selected_plugin
                            .map(|i| if i > 0 { i - 1 } else { app.plugins.len() - 1 })
                            .or(Some(0));
                    }
                    crate::app::Tab::Load => {
                        app.selected_menu = app.selected_menu
                            .map(|i| if i > 0 { i - 1 } else { app.menu_items.len() - 1 })
                            .or(Some(0));
                    }
                    _ => {}
                }
            }
            KeyCode::Down => {
                match app.current_tab {
                    crate::app::Tab::Plugins => {
                        app.selected_plugin = app.selected_plugin
                            .map(|i| if i < app.plugins.len() - 1 { i + 1 } else { 0 })
                            .or(Some(0));
                    }
                    crate::app::Tab::Load => {
                        app.selected_menu = app.selected_menu
                            .map(|i| if i < app.menu_items.len() - 1 { i + 1 } else { 0 })
                            .or(Some(0));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    Ok(())
} 