use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemImpl, parse_macro_input};

use crate::controller::routes::*;

pub fn expand_controller_routes(_: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemImpl);
    let parsed = parse_routes(item.clone());
    let generated = generate_controller_routes(&item.self_ty, parsed);

    let expanded = quote! {
        #item
        #generated
    };

    TokenStream::from(expanded)
}
