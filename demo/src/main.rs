use std::io;
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
use upaya_core::{Result, UpayaPlugin};
use upaya_plugin::DefaultPluginLoader;
use upaya_demo::ExamplePlugin;
use upaya_tui::{app, event, ui};

#[tokio::main]
async fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let mut app = app::App::new();

    // Initialize plugin loader
    let _loader = DefaultPluginLoader::new();
    let example_plugin = ExamplePlugin::new();
    app.update_plugins(vec![example_plugin.metadata()]);

    // Run the application
    let mut reader = EventStream::new();
    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        let timeout = Duration::from_millis(200);
        if crossterm::event::poll(timeout)? {
            if let Some(Ok(event)) = reader.next().await {
                if let Event::Key(key) = event {
                    if key.code == crossterm::event::KeyCode::Char('q') {
                        break;
                    }
                }
                event::handle_event(event, &mut app)?;
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
} 