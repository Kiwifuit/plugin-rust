use anyhow::Context;
use libloading::{Library, Symbol};
use log::{error, info};
use tplug_common::plugin::{Plugin, PluginImpl};

#[allow(improper_ctypes_definitions)]
type PluginInitFn = unsafe extern "C" fn() -> Box<dyn PluginImpl>;

#[derive(Default)]
struct PluginManager<T: PluginImpl> {
    plugins: Vec<Plugin<T>>,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    // let mut plugins = PluginManager::default();
    let plugin_path = "./hello.so";

    info!("Loading plugin: {}", plugin_path);
    let plugin_obj = unsafe { Library::new(plugin_path).context("while loading library") }?;
    let plugin_init: Symbol<'_, PluginInitFn> =
        unsafe { plugin_obj.get(b"plugin_init") }.context("while getting `plugin_init` symbol")?;

    let plugin = unsafe { plugin_init() };

    info!("Initializing: {} v{}", plugin.name(), plugin.version());
    if let Err(e) = plugin.init() {
        error!("Unable to load plugin {}: {}", plugin.name(), e);
    }

    Ok(())
}
