use proc_macro2::TokenStream;
use quote::quote;

use crate::di::DependencyStuctInput;

pub fn generate_provider_trait(parsed: &DependencyStuctInput) -> TokenStream {
    let struct_name = &parsed.struct_name;

    quote! {
        impl ::sword::core::Provider for #struct_name {}

        impl TryFrom<&::sword::core::State> for #struct_name {
            type Error = ::sword::errors::DependencyInjectionError;

            fn try_from(state: &::sword::core::State) -> Result<Self, Self::Error> {
                state.get::<Self>()
                    .map_err(|_| ::sword::errors::DependencyInjectionError::DependencyNotFound {
                        type_name: stringify!(#struct_name).to_string(),
                    })
            }
        }
    }
}
