use quote::quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn plugin_main(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let plugin_init = &input_fn.block;

    quote!{
        #[no_mangle]
        pub static PLUGIN_METADATA: tplug_common::PluginMetadata<'static> = tplug_common::PluginMetadata {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            init: || { #plugin_init }
        };
    }.into()
}
