#[cfg(feature = "plugin")]
pub struct PluginMetadata<'a> {
    name: &'a str,
    version: &'a str,
    init: fn() -> Box<dyn std::any::Any>,
}
