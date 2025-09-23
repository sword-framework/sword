use proc_macro2::Ident;
use syn::{ItemStruct, Type};

pub fn collect_controller_fields(item: &ItemStruct) -> Vec<(&Ident, &Type)> {
    match &item.fields {
        syn::Fields::Named(named_fields) => named_fields
            .named
            .iter()
            .filter_map(|field| field.ident.as_ref().map(|ident| (ident, &field.ty)))
            .collect(),
        _ => Vec::new(),
    }
}
