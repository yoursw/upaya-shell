mod app;
mod event;
mod ui;

use std::{io, panic};
use std::time::Duration;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::StreamExt;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use upaya_core::Result;
use upaya_plugin::DefaultPluginLoader;
use upaya_tui::{run_app};

#[tokio::main]
async fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create application state
    let mut app = app::App::new();

    // Initialize plugin loader
    let loader = DefaultPluginLoader::new();
    
    // Load example plugin
    let example_plugin = upaya_demo::ExamplePlugin::new();
    app.add_message("Example plugin loaded".to_string());

    // Update plugin list
    app.update_plugins(vec![example_plugin.metadata()]);

    // Create app and run it
    let res = run_app(&mut terminal, app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: app::App,
) -> Result<()> {
    let mut reader = EventStream::new();
    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        let timeout = Duration::from_millis(200);
        if crossterm::event::poll(timeout)? {
            if let Some(Ok(event)) = reader.next().await {
                if let Event::Key(key) = event {
                    if key.code == crossterm::event::KeyCode::Char('q') {
                        return Ok(());
                    }
                }
                event::handle_event(event, &mut app)?;
            }
        }
    }
} 