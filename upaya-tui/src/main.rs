mod app;
mod event;
mod ui;

use std::io;
use std::time::Duration;
use std::sync::{Arc, Mutex};
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

#[tokio::main]
async fn main() -> Result<()> {
    // Create application state
    let app = Arc::new(Mutex::new(app::App::new()));

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Load example plugin
    let example_plugin = upaya_demo::ExamplePlugin::new();
    if let Ok(mut app) = app.lock() {
        app.update_plugins(vec![(example_plugin.metadata(), Arc::new(example_plugin))]);
    }

    // Run the app
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
        eprintln!("{:?}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: Arc<Mutex<app::App>>,
) -> Result<()> {
    let mut reader = EventStream::new();
    loop {
        terminal.draw(|f| {
            if let Ok(app) = app.lock() {
                ui::render(f, &app);
            }
        })?;

        let timeout = Duration::from_millis(200);
        if crossterm::event::poll(timeout)? {
            if let Some(Ok(event)) = reader.next().await {
                if let Event::Key(key) = event {
                    if key.code == crossterm::event::KeyCode::Char('q') {
                        return Ok(());
                    }
                }
                if let Ok(mut app) = app.lock() {
                    event::handle_event(event, &mut app)?;
                }
            }
        }
    }
}
