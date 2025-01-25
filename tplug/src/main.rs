use anyhow::Context;
use libloading::{Library, Symbol};
use log::{info, Log};
use tplug_common::plugins::PluginMetadata;

type Plugin = *const PluginMetadata<'static>;

fn main() -> anyhow::Result<()> {
    colog::init();

    let file = "./target/debug/libtplug_plugin.so";

    let plugin = unsafe { Library::new(file).context("while opening plugin") }?;
    let plugin_meta: Symbol<Plugin> =
        unsafe { plugin.get(b"PLUGIN_METADATA") }.context("while fetching plugin metadata")?;

    let a = unsafe { &**plugin_meta };

    info!("Loading plugin: {}", a);
    let logger = Box::new(log::logger());
    let res = (a.init)(logger);

    dbg!(res);

    // tplug_common::plugins::PluginMetadata;

    Ok(())
}
