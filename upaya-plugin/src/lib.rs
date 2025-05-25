//! Plugin interface implementation for Upaya Shell
//!
//! This crate provides the implementation of plugin loading and execution.

use std::path::PathBuf;
use upaya_core::{Result, UpayaPlugin, PluginMetadata, PluginSource, PluginLoader, UpayaError};

/// Plugin loader implementation
pub struct DefaultPluginLoader {
    plugins: Vec<Box<dyn UpayaPlugin>>,
}

impl DefaultPluginLoader {
    /// Create a new plugin loader
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }
}

impl PluginLoader for DefaultPluginLoader {
    fn load_plugin(&self, source: PluginSource) -> Result<Box<dyn UpayaPlugin>> {
        match source {
            PluginSource::Git { url, branch } => {
                // TODO: Implement Git plugin loading
                Err(UpayaError::Plugin("Git loading not implemented".into()))
            }
            PluginSource::Http { url } => {
                // TODO: Implement HTTP plugin loading
                Err(UpayaError::Plugin("HTTP loading not implemented".into()))
            }
            PluginSource::Local { path } => {
                Err(UpayaError::Plugin("Local loading not implemented".into()))
            }
        }
    }

    fn list_plugins(&self) -> Result<Vec<PluginMetadata>> {
        Ok(self.plugins.iter().map(|p| p.metadata()).collect())
    }

    fn unload_plugin(&self, name: &str) -> Result<()> {
        // TODO: Implement plugin unloading
        Ok(())
    }
}
