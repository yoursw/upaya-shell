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
            Constraint::Length(3),  // Input area
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
        Tab::Load => render_load_tab(f, app, main_chunks[1]),
        Tab::Settings => render_settings_tab(f, app, main_chunks[1]),
    }

    // Render input area
    render_input(f, app, chunks[2]);
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
            Span::styled("Load", if app.current_tab == Tab::Load {
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
    let items: Vec<ListItem> = app.plugins
        .iter()
        .enumerate()
        .map(|(i, plugin)| {
            let style = if Some(i) == app.selected_plugin {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            ListItem::new(format!("{} - {}", plugin.name, plugin.version))
                .style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Plugins").borders(Borders::ALL));
    f.render_widget(list, area);
}

fn render_load_tab(
    f: &mut Frame,
    app: &App,
    area: Rect,
) {
    let items: Vec<ListItem> = app.menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if Some(i) == app.selected_menu {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            ListItem::new(item.title.as_str()).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Load Options").borders(Borders::ALL));
    f.render_widget(list, area);
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

fn render_input(f: &mut Frame, app: &App, area: Rect) {
    let input = Paragraph::new(app.input.as_str())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, area);
} 