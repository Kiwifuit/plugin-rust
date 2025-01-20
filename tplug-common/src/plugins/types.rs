// #[cfg(feature = "plugin-manager")]
use std::fmt::Display;

pub struct PluginMetadata<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub init: fn() -> Box<dyn std::any::Any>,
}

// #[cfg(feature = "plugin-manager")]
impl Display for PluginMetadata<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plugin {} v{}", self.name, self.version)
    }
}
