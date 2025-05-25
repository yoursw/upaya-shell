//! Terminal User Interface for Upaya Shell
//! 
//! This crate provides the terminal interface for Upaya Shell.

use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use upaya_core::{Result, PluginMetadata, UpayaError};

/// TUI application state
pub struct App {
    plugins: Vec<PluginMetadata>,
    selected_plugin: Option<usize>,
    input: String,
    messages: Vec<String>,
}

impl App {
    /// Create a new TUI application
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            selected_plugin: None,
            input: String::new(),
            messages: Vec::new(),
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
}

/// Run the TUI application
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char(c) => app.input.push(c),
                KeyCode::Backspace => {
                    app.input.pop();
                }
                KeyCode::Enter => {
                    // TODO: Handle command execution
                    app.add_message(format!("Executed: {}", app.input));
                    app.input.clear();
                }
                _ => {}
            }
        }
    }
}

/// Draw the TUI interface
fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Title
    let title = Paragraph::new("Upaya Shell")
        .style(Style::default().add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Main content
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[1]);

    // Plugin list
    let plugins: Vec<ListItem> = app
        .plugins
        .iter()
        .map(|p| {
            ListItem::new(Line::from(vec![
                Span::raw(&p.name),
                Span::raw(" "),
                Span::styled(&p.version, Style::default().fg(Color::Yellow)),
            ]))
        })
        .collect();

    let plugins = List::new(plugins)
        .block(Block::default().title("Plugins").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    f.render_widget(plugins, main_chunks[0]);

    // Message log
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| ListItem::new(m.as_str()))
        .collect();

    let messages = List::new(messages)
        .block(Block::default().title("Messages").borders(Borders::ALL));
    f.render_widget(messages, main_chunks[1]);

    // Input
    let input = Paragraph::new(app.input.as_str())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[2]);
}

/// Initialize the terminal
pub fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Restore the terminal
pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
} 