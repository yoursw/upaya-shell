//! Core functionality for Upaya Shell
//! 
//! This crate provides the core interfaces and types for the Upaya Shell system.

use std::path::PathBuf;
use thiserror::Error;
use serde::{Serialize, Deserialize};

/// Result type for Upaya operations
pub type Result<T> = std::result::Result<T, UpayaError>;

/// Core error type for Upaya operations
#[derive(Error, Debug)]
pub enum UpayaError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Plugin error: {0}")]
    Plugin(String),

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
}

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub permissions: PluginPermissions,
}

/// Plugin permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginPermissions {
    pub filesystem: bool,
    pub network: bool,
    pub system: bool,
}

/// Plugin source location
#[derive(Debug, Clone)]
pub enum PluginSource {
    Git {
        url: String,
        branch: Option<String>,
    },
    Http {
        url: String,
    },
    Local {
        path: PathBuf,
    },
}

/// Plugin interface trait
pub trait UpayaPlugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> PluginMetadata;

    /// Initialize the plugin
    fn init(&mut self) -> Result<()>;

    /// Execute the plugin
    fn execute(&self, args: &[String]) -> Result<String>;

    /// Cleanup resources
    fn cleanup(&mut self) -> Result<()>;
}

/// Plugin loader trait
pub trait PluginLoader: Send + Sync {
    /// Load a plugin from a source
    fn load_plugin(&self, source: PluginSource) -> Result<Box<dyn UpayaPlugin>>;

    /// List available plugins
    fn list_plugins(&self) -> Result<Vec<PluginMetadata>>;

    /// Unload a plugin
    fn unload_plugin(&self, name: &str) -> Result<()>;
} 