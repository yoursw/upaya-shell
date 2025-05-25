//! Event handling

use crossterm::event::{Event, KeyCode};
use upaya_core::{Result, UpayaPlugin};
use crate::app::App;

/// Handle terminal events
pub fn handle_event(event: Event, app: &mut App) -> Result<()> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Char('q') => return Ok(()),
            KeyCode::Tab => {
                app.current_tab = match app.current_tab {
                    crate::app::Tab::Plugins => crate::app::Tab::Settings,
                    crate::app::Tab::Settings => crate::app::Tab::Plugins,
                };
            }
            KeyCode::Enter => {
                match app.current_tab {
                    crate::app::Tab::Plugins => {
                        if let Some(idx) = app.selected_plugin {
                            // Get plugin info first
                            let plugin_info = app.plugins.get(idx).map(|(metadata, plugin)| {
                                (metadata.name.clone(), plugin.clone())
                            });
                            
                            // Then use the info
                            if let Some((name, plugin)) = plugin_info {
                                app.add_debug(format!("Executing plugin: {}", name));
                                app.clear_plugin_output();
                                let args: Vec<String> = vec![];
                                match plugin.execute(&args) {
                                    Ok(output) => app.add_plugin_output(output),
                                    Err(e) => app.add_plugin_output(format!("Error: {}", e)),
                                }
                            }
                        }
                    }
                    crate::app::Tab::Settings => {
                        app.add_debug("Settings updated".to_string());
                    }
                }
            }
            KeyCode::Up => {
                if app.current_tab == crate::app::Tab::Plugins {
                    app.selected_plugin = app.selected_plugin
                        .map(|i| if i > 0 { i - 1 } else { app.plugins.len() - 1 })
                        .or(Some(0));
                }
            }
            KeyCode::Down => {
                if app.current_tab == crate::app::Tab::Plugins {
                    app.selected_plugin = app.selected_plugin
                        .map(|i| if i < app.plugins.len() - 1 { i + 1 } else { 0 })
                        .or(Some(0));
                }
            }
            _ => {}
        }
    }
    Ok(())
} 