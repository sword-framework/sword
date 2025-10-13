use proc_macro2::TokenStream;
use quote::quote;

use crate::{di::DependencyStuctInput, shared::*};

pub fn generate_injectable_trait(input: &DependencyStuctInput) -> TokenStream {
    let field_extractions = generate_field_extraction_from_state(&input.fields);
    let field_assignments = generate_struct_field_assignments(&input.fields);

    let struct_name = &input.struct_name;

    let type_ids = input.fields.iter().map(|(_, ty)| {
        quote! {
            std::any::TypeId::of::<#ty>()
        }
    });

    quote! {
        impl ::sword::core::Injectable for #struct_name {
            fn build(state: &::sword::core::State) -> Result<Self, ::sword::errors::DependencyInjectionError> {
                Self::try_from(state)
            }

            fn dependencies() -> Vec<std::any::TypeId> {
                vec![
                    #(#type_ids),*
                ]
            }
        }

        impl TryFrom<&::sword::core::State> for #struct_name {
            type Error = ::sword::errors::DependencyInjectionError;

            fn try_from(state: &::sword::core::State) -> Result<Self, Self::Error> {
                #field_extractions

                Ok(Self {
                    #field_assignments
                })
            }
        }
    }
}
