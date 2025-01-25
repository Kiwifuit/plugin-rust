use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, FnArg, ItemFn, Token};

#[proc_macro_attribute]
pub fn plugin_main(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let plugin_init = &input_fn.block;
    let plugin_init_args = &input_fn.sig.inputs;

    if let Some(err) = validate_plugin_args(plugin_init_args) {
        return quote! {
            compile_error!(#err);
        }
        .into();
    }

    quote!{
        #[no_mangle]
        pub static PLUGIN_METADATA: tplug_common::PluginMetadata<'static> = tplug_common::PluginMetadata {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            init: |#plugin_init_args| { #plugin_init }
        };
    }.into()
}

fn validate_plugin_args(args: &Punctuated<FnArg, Token![,]>) -> Option<&'static str> {
    if args.len() != 1 {
        return Some("Expected to have 1 argument");
    }

    // Get the first argument and check its type
    let arg = match &args[0] {
        FnArg::Typed(arg) => arg,
        _ => return Some("Invalid function signature. Expected an argument of type `&str`."),
    };

    // Check if the argument is a reference
    let ref_type = match &*arg.ty {
        syn::Type::Reference(ref_type) => ref_type,
        _ => return Some("The function must take an argument of type `&str`."),
    };

    // Check if the reference points to a `str`
    let last_segment = match &*ref_type.elem {
        syn::Type::Path(type_path) => type_path.path.segments.last(),
        _ => return Some("The function must take an argument of type `&str`."),
    };

    if last_segment.is_none() || last_segment.unwrap().ident != "str" {
        return Some("The function must take an argument of type `&str`.");
    }

    None
}
