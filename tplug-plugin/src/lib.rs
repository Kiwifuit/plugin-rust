#[no_mangle]
pub static PLUGIN_METADATA: tplug_common::PluginMetadata<'static> = tplug_common::PluginMetadata {
    name: env!("CARGO_PKG_NAME"),
    version: env!("CARGO_PKG_VERSION"),
    init: || Box::new(Plugin),
};

struct Plugin;
