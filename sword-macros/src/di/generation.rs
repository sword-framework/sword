use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::di::DependencyStuctInput;

pub fn generate_clone_impl(parsed: &DependencyStuctInput) -> TokenStream {
    let struct_name = &parsed.struct_name;
    let field_names: Vec<&Ident> =
        parsed.fields.iter().map(|(name, _)| name).collect();

    quote! {
        impl ::std::clone::Clone for #struct_name {
            fn clone(&self) -> Self {
                Self {
                    #(#field_names: self.#field_names.clone()),*
                }
            }
        }
    }
}
