#[cfg(feature = "plugin-manager")]
use std::fmt::Display;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
pub type Plugin = Box<dyn std::any::Any>;

pub struct PluginMetadata<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub init: fn(Box<dyn log::Log>) -> Result<Plugin>,
}

#[cfg(feature = "plugin-manager")]
impl Display for PluginMetadata<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plugin {} v{}", self.name, self.version)
    }
}
