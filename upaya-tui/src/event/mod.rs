//! Event handling

use crossterm::event::{Event, KeyCode, KeyModifiers};
use upaya_core::{Result, UpayaPlugin};
use crate::app::{App, PluginViewState};

/// Handle terminal events
pub fn handle_event(event: Event, app: &mut App) -> Result<()> {
    // First, let plugins handle the event if we're in plugin view
    if app.current_tab == crate::app::Tab::Plugins &&
       (app.plugin_view_state == PluginViewState::Input || app.plugin_view_state == PluginViewState::Output) {
        if let Some(idx) = app.selected_plugin {
            if let Some((_, plugin)) = app.plugins.get(idx) {
                if let Ok(true) = plugin.handle_event(&event) {
                    return Ok(());
                }
            }
        }
    }

    // If plugin didn't handle the event, handle it in the main app
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
                        match app.plugin_view_state {
                            PluginViewState::List => {
                                if app.selected_plugin.is_some() {
                                    app.plugin_view_state = PluginViewState::Details;
                                }
                            }
                            PluginViewState::Details => {
                                app.plugin_view_state = PluginViewState::Input;
                            }
                            _ => {}
                        }
                    }
                    crate::app::Tab::Settings => {
                        app.add_debug("Settings updated".to_string());
                    }
                }
            }
            KeyCode::Up => {
                if app.current_tab == crate::app::Tab::Plugins {
                    match app.plugin_view_state {
                        PluginViewState::List => {
                            app.selected_plugin = app.selected_plugin
                                .map(|i| if i > 0 { i - 1 } else { app.plugins.len() - 1 })
                                .or(Some(0));
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::Down => {
                if app.current_tab == crate::app::Tab::Plugins {
                    match app.plugin_view_state {
                        PluginViewState::List => {
                            app.selected_plugin = app.selected_plugin
                                .map(|i| if i < app.plugins.len() - 1 { i + 1 } else { 0 })
                                .or(Some(0));
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::Esc => {
                if app.current_tab == crate::app::Tab::Plugins {
                    match app.plugin_view_state {
                        PluginViewState::Details | PluginViewState::Input | PluginViewState::Output => {
                            app.plugin_view_state = PluginViewState::List;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}
