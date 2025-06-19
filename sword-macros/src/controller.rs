use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};

pub fn expand_controller_derive(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;

    let expanded = quote! {
        impl ::sword::controller::ControllerKind for #struct_name {
            fn name() -> &'static str {
                stringify!(#struct_name)
            }

            fn file_path() -> &'static str {
                file!()
            }
        }
    };

    TokenStream::from(expanded)
}
