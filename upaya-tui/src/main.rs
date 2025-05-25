use std::error::Error;
use upaya_core::Result;
use upaya_plugin::DefaultPluginLoader;
use upaya_tui::{App, init_terminal, restore_terminal, run_app};

fn main() -> Result<()> {
    // Initialize terminal
    let mut terminal = init_terminal()?;

    // Create application state
    let mut app = App::new();

    // Initialize plugin loader
    let loader = DefaultPluginLoader::new();
    
    // Load example plugin
    let example_plugin = upaya_demo::ExamplePlugin::new();
    app.add_message("Example plugin loaded".to_string());

    // Update plugin list
    app.update_plugins(vec![example_plugin.metadata()]);

    // Run the application
    let result = run_app(&mut terminal, app);

    // Restore terminal
    restore_terminal(&mut terminal)?;

    result
} 