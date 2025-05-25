//! Example plugin for Upaya Shell
//! 
//! This crate provides an example plugin implementation.

use upaya_core::{Result, UpayaPlugin, PluginMetadata, PluginPermissions};

/// Example plugin implementation
pub struct ExamplePlugin {
    metadata: PluginMetadata,
}

impl ExamplePlugin {
    /// Create a new example plugin
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                name: "example".to_string(),
                version: "0.1.0".to_string(),
                description: "An example plugin for Upaya Shell".to_string(),
                author: "Your SW <admin@kusala.tech>".to_string(),
                permissions: PluginPermissions {
                    filesystem: false,
                    network: false,
                    system: false,
                },
            },
        }
    }
}

impl UpayaPlugin for ExamplePlugin {
    fn metadata(&self) -> PluginMetadata {
        self.metadata.clone()
    }

    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn execute(&self, args: &[String]) -> Result<String> {
        Ok(format!("Example plugin executed with args: {:?}", args))
    }

    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }
} 