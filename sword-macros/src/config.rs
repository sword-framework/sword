use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;
use syn::{Expr, ItemStruct, Lit, Meta, parse_macro_input};

pub fn expand_config_struct(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let meta = parse_macro_input!(attr as Meta);

    let toml_key_str = match meta {
        Meta::NameValue(nv) if nv.path.is_ident("key") => {
            if let Expr::Lit(expr) = nv.value {
                if let Lit::Str(lit_str) = expr.lit {
                    lit_str.value()
                } else {
                    emit_error!(expr, "Expected a string literal for the toml key");
                    return TokenStream::new();
                }
            } else {
                emit_error!(nv.value, "Expected a literal for the toml key");
                return TokenStream::new();
            }
        }
        _ => {
            emit_error!(meta, "Expected a `key = \"...\"` attribute");
            return TokenStream::new();
        }
    };

    let struct_name = &input.ident;

    let expanded = quote! {
        #input

        impl ::sword::application::config::ConfigItem for #struct_name {
            fn toml_key() -> &'static str {
                #toml_key_str
            }
        }
    };

    TokenStream::from(expanded)
}
