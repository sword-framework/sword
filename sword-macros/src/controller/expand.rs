use proc_macro::TokenStream;
use quote::quote;
use syn::ItemStruct;

use crate::controller::{
    generation::generate_controller_builder, parsing::parse_controller_input,
};

pub fn expand_controller(
    attr: TokenStream,
    item: TokenStream,
) -> Result<TokenStream, syn::Error> {
    let input = syn::parse::<ItemStruct>(item.clone())?;
    let parsed_input = parse_controller_input(attr, item)?;
    let builder = generate_controller_builder(&parsed_input);

    let expanded = quote! {
        #input
        #builder
    };

    Ok(TokenStream::from(expanded))
}
