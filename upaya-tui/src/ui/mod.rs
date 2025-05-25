//! UI rendering components

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::app::{App, Tab};

/// Render the TUI interface
pub fn render(f: &mut Frame, app: &App) {
    // Create the main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(0),     // Main content
            Constraint::Length(10), // Debug console
        ])
        .split(f.size());

    // Render title
    let title = Paragraph::new("Upaya TUI")
        .style(Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Render main content based on current tab
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Tabs
            Constraint::Min(0),     // Content
        ])
        .split(chunks[1]);

    // Render tabs
    render_tabs(f, app, main_chunks[0]);

    // Render content based on current tab
    match app.current_tab {
        Tab::Plugins => render_plugins_tab(f, app, main_chunks[1]),
        Tab::Settings => render_settings_tab(f, app, main_chunks[1]),
    }

    // Render debug console
    render_debug_console(f, app, chunks[2]);
}

fn render_tabs(f: &mut Frame, app: &App, area: Rect) {
    let tabs = vec![
        Line::from(vec![
            Span::styled("Plugins", if app.current_tab == Tab::Plugins {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            }),
            Span::raw(" | "),
            Span::styled("Settings", if app.current_tab == Tab::Settings {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            }),
        ]),
    ];

    let tabs = Paragraph::new(tabs)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(tabs, area);
}

fn render_plugins_tab(
    f: &mut Frame,
    app: &App,
    area: Rect,
) {
    // Split the area into two parts: plugin list and plugin view
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(area);

    // Render plugin list
    let items: Vec<ListItem> = app.plugins
        .iter()
        .enumerate()
        .map(|(i, (metadata, _))| {
            let style = if Some(i) == app.selected_plugin {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            ListItem::new(format!("{} - {}", metadata.name, metadata.version))
                .style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Plugins").borders(Borders::ALL));
    f.render_widget(list, chunks[0]);

    // If a plugin is selected, render its view
    if let Some(idx) = app.selected_plugin {
        if let Some((metadata, _)) = app.plugins.get(idx) {
            render_plugin_view(f, metadata, app, chunks[1]);
        }
    }
}

fn render_plugin_view(f: &mut Frame, metadata: &upaya_core::PluginMetadata, app: &App, area: Rect) {
    let view = vec![
        Line::from(format!("Plugin: {}", metadata.name)),
        Line::from(format!("Version: {}", metadata.version)),
        Line::from("Press Enter to execute"),
        Line::from(""),
        Line::from("Output:"),
        Line::from("------"),
        Line::from(if !app.plugin_output.is_empty() {
            app.plugin_output.as_str()
        } else {
            "No output yet"
        }),
    ];

    let view = Paragraph::new(view)
        .block(Block::default().title("Plugin View").borders(Borders::ALL));
    f.render_widget(view, area);
}

fn render_settings_tab(
    f: &mut Frame,
    app: &App,
    area: Rect,
) {
    let settings = vec![
        Line::from("Settings"),
        Line::from(""),
        Line::from("Press 'q' to quit"),
    ];

    let settings = Paragraph::new(settings)
        .block(Block::default().title("Settings").borders(Borders::ALL));
    f.render_widget(settings, area);
}

fn render_debug_console(f: &mut Frame, app: &App, area: Rect) {
    let debug_lines: Vec<Line> = app.debug_output
        .iter()
        .map(|line| Line::from(line.as_str()))
        .collect();

    let debug = Paragraph::new(debug_lines)
        .block(Block::default().title("Debug Console").borders(Borders::ALL))
        .style(Style::default().fg(Color::Gray));
    f.render_widget(debug, area);
}
