use upaya_core::{Result, UpayaPlugin};
use upaya_tui::{App, init_terminal, restore_terminal, run_app};
use upaya_demo::ExamplePlugin;

fn main() -> Result<()> {
    // Initialize terminal
    let mut terminal = init_terminal()?;

    // Create application state
    let mut app = App::new();

    // Create and initialize the example plugin
    let mut plugin = ExamplePlugin::new();
    plugin.init()?;
    app.add_message("Example plugin initialized".to_string());

    // Update plugin list
    app.update_plugins(vec![plugin.metadata()]);

    // Execute the plugin with some example arguments
    let args = vec!["--demo".to_string(), "test".to_string()];
    plugin.execute(&args)?;
    app.add_message(format!("Example plugin executed with args: {:?}", args));

    // Run the TUI application
    let result = run_app(&mut terminal, app);

    // Cleanup
    plugin.cleanup()?;

    // Restore terminal
    restore_terminal(&mut terminal)?;

    result
} 