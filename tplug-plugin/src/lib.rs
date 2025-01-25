//#[no_mangle]
//pub static PLUGIN_METADATA: tplug_common::plugins::PluginMetadata<'static> =
//    tplug_common::plugins::PluginMetadata {
//        name: env!("CARGO_PKG_NAME"),
//        version: env!("CARGO_PKG_VERSION"),
//        init: || {
//            println!("Hello world!");
//            Box::new(Plugin)
//        },
//    };

use log::info;
use thiserror::Error;

#[derive(Debug, Error)]
enum PluginError {
    #[error("unable to set logger: {0}")]
    LogInit(#[from] log::SetLoggerError),
}

struct Plugin;

#[tplug_common::plugin_main]
fn init(logger: Box<dyn log::Log>) -> Result<Box<Plugin>, PluginError> {
    log::set_boxed_logger(logger)?;

    info!("Hello world from the plugin");

    Ok(Box::new(Plugin))
}
