use proc_macro::TokenStream;
use quote::quote;
use syn::ItemStruct;

use crate::di::{
    generate_clone_impl, parse_dependency_struct_input,
    provider::generate_provider_trait,
};

pub fn expand_provider(
    attr: TokenStream,
    item: TokenStream,
) -> Result<TokenStream, syn::Error> {
    let input = syn::parse::<ItemStruct>(item.clone())?;
    let parsed = parse_dependency_struct_input(attr, item)?;

    let provider_impl = generate_provider_trait(&parsed);
    let clone_impl = generate_clone_impl(&parsed);

    let mut expanded = quote! {
        #input
        #provider_impl
    };

    if parsed.derive_clone {
        expanded.extend(quote! {
            #clone_impl
        });
    }

    Ok(expanded.into())
}
