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

struct Plugin;

#[tplug_common::plugin_main]
fn init() -> Box<Plugin> {
    println!("Hello, world!");

    Box::new(Plugin)
}
