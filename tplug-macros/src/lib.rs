use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, FnArg, GenericArgument, ItemFn, PathArguments,
    Token, Type, TypePath, TypeTraitObject,
};

#[proc_macro_attribute]
pub fn plugin_main(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let plugin_init = &input_fn.block;
    let plugin_init_args = &input_fn.sig.inputs;

    let rustc_version_major = env!("RUSTC_VERSION_MAJOR");
    let rustc_version_minor = env!("RUSTC_VERSION_MINOR");
    let rustc_version_patch = env!("RUSTC_VERSION_PATCH");

    if let Some(err) = validate_plugin_args(plugin_init_args) {
        return syn::Error::new_spanned(plugin_init_args, err)
            .to_compile_error()
            .into();
    }

    quote!{
        #[no_mangle]
        pub static PLUGIN_METADATA: ::tplug_common::PluginMetadata<'static> = ::tplug_common::PluginMetadata {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            compiled_for_rustc_version: (
                #rustc_version_major,
                #rustc_version_minor,
                #rustc_version_patch,
            ),
            init: |#plugin_init_args| #plugin_init
        };
    }.into()
}

fn validate_plugin_args(args: &Punctuated<FnArg, Token![,]>) -> Option<&'static str> {
    if args.len() != 1 {
        return Some("Expected to have at least 1 argument");
    }

    args.iter()
        .next()
        .into_iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(typed_arg) => Some(&typed_arg.ty),
            _ => None,
        })
        .next()
        .map_or(
            Some("Expected plugin main to accept at most 1 argument"),
            |arg_type| {
                if !is_box_dyn_log(arg_type) {
                    return Some("Expected function argument to be of type `Box<dyn log::Log>`");
                }

                None
            },
        )
}

fn is_box_dyn_log(ty: &Type) -> bool {
    match ty {
        Type::Path(TypePath { path, .. }) => Some(path),
        _ => None,
    }
    .and_then(|path| {
        if let Some(PathArguments::AngleBracketed(path_args)) =
            path.segments.first().map(|seg| &seg.arguments)
        {
            if path_args.args.iter().any(|generic_arg| {
                matches!(
                    generic_arg,
                    GenericArgument::Type(Type::TraitObject(generic_type_bound))
                        if find_log_trait(generic_type_bound).is_some()

                )
            }) {
                return Some(());
            }
        }

        None
    })
    .is_some()
}

fn find_log_trait(generic_type_bound: &TypeTraitObject) -> Option<&syn::TraitBound> {
    generic_type_bound
        .bounds
        .iter()
        // Only get traits
        .filter_map(|type_parameter| match type_parameter {
            syn::TypeParamBound::Trait(trait_bound) => Some(trait_bound),
            _ => None,
        })
        // Find the `Log` trait
        .find(|trait_bound| {
            trait_bound
                .path
                .segments
                .last()
                .is_some_and(|trait_segment| trait_segment.ident == "Log")
        })
}
