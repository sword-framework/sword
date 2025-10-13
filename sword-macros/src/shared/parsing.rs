use syn::{Ident, ItemStruct, Type};

pub fn collect_struct_fields(item: &ItemStruct) -> Vec<(Ident, Type)> {
    match &item.fields {
        syn::Fields::Named(named_fields) => named_fields
            .named
            .iter()
            .filter_map(|field| {
                field
                    .ident
                    .as_ref()
                    .map(|ident| (ident.clone(), field.ty.clone()))
            })
            .collect(),
        _ => Vec::new(),
    }
}
