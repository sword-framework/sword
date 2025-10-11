use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    injectable::InjectableInput,
    shared::{
        generate_field_extraction_from_state, generate_struct_field_assignments,
    },
};

pub fn generate_injectable_trait(input: &InjectableInput) -> TokenStream {
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
                #field_extractions

                Ok(Self {
                    #field_assignments
                })
            }

            fn dependencies() -> Vec<std::any::TypeId> {
                vec![
                    #(#type_ids),*
                ]
            }
        }
    }
}

pub fn generate_clone_impl(input: &InjectableInput) -> TokenStream {
    let struct_name = &input.struct_name;
    let field_clones = input.fields.iter().map(|(name, _)| {
        quote! {
            #name: self.#name.clone()
        }
    });

    quote! {
        impl Clone for #struct_name {
            fn clone(&self) -> Self {
                Self {
                    #(#field_clones),*
                }
            }
        }
    }
}
