use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};

use crate::controller::{
    generation::generate_controller_builder, parsing::parse_controller_input,
};

pub fn expand_controller(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = item.clone();
    let input = parse_macro_input!(input as ItemStruct);

    let parsed_input = match parse_controller_input(attr, item) {
        Ok(ci) => ci,
        Err(e) => {
            emit_error!("{}", e);
            return TokenStream::new();
        }
    };

    let builder = generate_controller_builder(&parsed_input);

    let expanded = quote! {
        #input
        #builder
    };

    TokenStream::from(expanded)
}
