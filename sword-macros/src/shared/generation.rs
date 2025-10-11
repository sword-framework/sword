use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

pub fn generate_field_extraction_from_state(
    fields: &[(Ident, Type)],
) -> TokenStream {
    let extractions = fields.iter().map(|(field_name, field_type)| {
        let type_str = quote!(#field_type).to_string();

        quote! {
            let #field_name = state.get::<#field_type>().map_err(|_| {
                ::sword::errors::DependencyInjectionError::DependencyNotFound {
                    type_name: #type_str.to_string(),
                }
            })?;
        }
    });

    quote! {
        #(#extractions)*
    }
}

pub fn generate_struct_field_assignments(fields: &[(Ident, Type)]) -> TokenStream {
    let assignments = fields.iter().map(|(name, _)| {
        quote! { #name }
    });

    quote! {
        #(#assignments),*
    }
}
