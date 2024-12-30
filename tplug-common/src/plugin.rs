use std::{error::Error, ops::Deref};

use log::{debug, error};

/// Represents a loaded plugin. This type
/// is transparent, and therefore acts like
/// a `PluginImpl`, and can be used as such.
#[repr(transparent)]
pub struct Plugin<T: PluginImpl> {
    plugin: T,
}

impl<T: PluginImpl> Plugin<T> {
    pub fn new(plugin: T) -> Self {
        Self { plugin }
    }
}

impl<T: PluginImpl> Drop for Plugin<T> {
    fn drop(&mut self) {
        debug!("Cleaning up resources for {}", self.plugin.name());
        if let Err(e) = self.plugin.on_cleanup() {
            error!(
                "An error occured while trying to clean plugin {}: {}",
                self.plugin.name(),
                e
            );
        }
    }
}

impl<T: PluginImpl> Deref for Plugin<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.plugin
    }
}

// impl<T: PluginImpl> std::ops::DerefMut for Plugin<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.plugin
//     }
// }

/// Represents a plugin for TPlug, an
/// experimental plugin system written in rust
pub trait PluginImpl {
    /// Name of the plugin. Defaults to
    /// the `package.name` set in the Cargo.toml
    fn name(&self) -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    /// Name of the plugin. Defaults to
    /// the `package.version` set in the Cargo.toml
    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// Initializes the plugin. This method
    /// is run by TPlug once during startup
    fn init(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    /// Cleans resources used by the plugin.
    /// This method is run by TPlug once
    /// the plugin is unloaded
    fn on_cleanup(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
