pub struct PluginMetadata<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub init: fn() -> Box<dyn std::any::Any>,
}
