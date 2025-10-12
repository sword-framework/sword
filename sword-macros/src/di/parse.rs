use proc_macro::TokenStream;
use syn::{Ident, ItemStruct, Type};

use crate::shared::collect_struct_fields;

pub struct DependencyStuctInput {
    pub struct_name: Ident,
    pub fields: Vec<(Ident, Type)>,
    pub derive_clone: bool,
}

pub fn parse_dependency_struct_input(
    attr: TokenStream,
    item: TokenStream,
) -> Result<DependencyStuctInput, syn::Error> {
    let input = syn::parse::<ItemStruct>(item)?;
    let struct_fields = collect_struct_fields(&input);

    Ok(DependencyStuctInput {
        struct_name: input.ident,
        fields: struct_fields,
        derive_clone: !attr.to_string().contains("no_derive_clone"),
    })
}
