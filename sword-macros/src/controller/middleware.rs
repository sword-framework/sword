use proc_macro2::TokenStream;
use quote::quote;

use syn::{
    Expr, Path, Token,
    parse::{Parse, ParseStream},
};

pub struct MiddlewareArgs {
    pub path: Path,
    pub config: Option<Expr>,
}

impl Parse for MiddlewareArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path: Path = input.parse()?;

        let mut config = None;

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;

            if input.peek(syn::Ident) && input.peek2(Token![=]) {
                let ident: syn::Ident = input.parse()?;

                if ident != "config" {
                    return Err(syn::Error::new(ident.span(), "expected 'config'"));
                }

                input.parse::<Token![=]>()?;
                config = Some(input.parse()?);
            }
        }

        Ok(MiddlewareArgs { path, config })
    }
}

pub fn expand_middleware_args(args: &MiddlewareArgs) -> TokenStream {
    let MiddlewareArgs { path, config } = args;

    match config {
        Some(config) => quote! {
            ::sword::__private::mw_with_state(
                app_state.clone(),
                |ctx: ::sword::http::Context, next: ::sword::middleware::Next| async move {
                    <#path>::handle(#config, ctx, next).await
                }
            )
        },
        None => quote! {
            ::sword::__private::mw_with_state(
                app_state.clone(),
                |ctx: ::sword::http::Context, next: ::sword::middleware::Next| async move {
                    <#path>::handle(ctx, next).await
                }
            )
        },
    }
}
