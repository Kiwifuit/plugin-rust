use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, FnArg, GenericArgument, ItemFn, PathArguments,
    Token, Type, TypePath,
};

#[proc_macro_attribute]
pub fn plugin_main(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let plugin_init = &input_fn.block;
    let plugin_init_args = &input_fn.sig.inputs;

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

/// Helper function to check if the type is `Box<dyn log::Log>`
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
                // dbg!(generic_arg);

                // if let GenericArgument::Type(Type::TraitObject(generic_type_bound)) = generic_arg {
                //     let a = generic_type_bound
                //         .dyn_token
                //         .and_then(|_| {
                //             generic_type_bound
                //                 .bounds
                //                 .iter()
                //                 .filter_map(|bound| match bound {
                //                     syn::TypeParamBound::Trait(trait_bound) => Some(trait_bound),
                //                     _ => None,
                //                 })
                //                 .find(|trait_bounds| {
                //                     trait_bounds
                //                         .path
                //                         .segments
                //                         .last()
                //                         .is_some_and(|trait_segment| trait_segment.ident == "Log")
                //                 })
                //         })
                //         .is_some();
                // }

                matches!(
                    generic_arg,
                    GenericArgument::Type(Type::TraitObject(generic_type_bound))
                        if generic_type_bound.dyn_token.and_then(|_| {
                            generic_type_bound
                                .bounds
                                .iter()
                                .filter_map(|bound| match bound {
                                    syn::TypeParamBound::Trait(trait_bound) => Some(trait_bound),
                                    _ => None,
                                })
                                .find(|trait_bounds| {
                                    trait_bounds
                                        .path
                                        .segments
                                        .last()
                                        .is_some_and(|trait_segment| trait_segment.ident == "Log")
                                })
                    }).is_some()

                )
            }) {
                return Some(());
            }
        }

        None
    })
    .is_some()

    // return true; // Short-circuit this function to always return true

    // Ensure the type is a `Box`
    // if let Type::Path(TypePath { path, .. }) = ty {
    //     if path.is_ident("Box") {
    //         // Ensure the type inside the Box is `dyn log::Log`
    //         if let Some(PathArguments::AngleBracketed(args)) =
    //             path.segments.first().map(|seg| &seg.arguments)
    //         {
    //             return args.args.iter().any(|arg|
    //                 matches!(
    //                     arg,
    //                     GenericArgument::Type(Type::TraitObject(obj))
    //                         if obj.bounds.get(0).is_some_and(|bound| {
    //                             matches!(
    //                                 bound,
    //                                 syn::TypeParamBound::Trait(trait_bound) if trait_bound.path.is_ident("log::Log")
    //                             )
    //                         })));
    //         }
    //     }
    // }

    // // if let Type::Path()

    // false
}
