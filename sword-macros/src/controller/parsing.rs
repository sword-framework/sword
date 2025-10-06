use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use syn::{Ident, ItemStruct, LitStr, Type};

use crate::middleware::parse::MiddlewareArgs;

pub struct ControllerInput {
    pub struct_name: Ident,
    pub base_path: String,
    pub fields: Vec<(Ident, Type)>,
    pub middlewares: Vec<MiddlewareArgs>,
}

pub fn parse_controller_input(
    attr: TokenStream,
    item: TokenStream,
) -> Result<ControllerInput, syn::Error> {
    let input = syn::parse::<ItemStruct>(item)?;
    let args = syn::parse::<LitStr>(attr)?;

    let mut middlewares = vec![];
    let fields = collect_controller_fields(&input);

    for attr in &input.attrs {
        if attr.path().is_ident("middleware") {
            match attr.parse_args::<MiddlewareArgs>() {
                Ok(args) => middlewares.push(args),
                Err(e) => emit_error!("Failed to parse middleware arguments: {}", e),
            }
        }
    }

    Ok(ControllerInput {
        base_path: args.value(),
        struct_name: input.ident,
        fields,
        middlewares,
    })
}

pub fn collect_controller_fields(item: &ItemStruct) -> Vec<(Ident, Type)> {
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
