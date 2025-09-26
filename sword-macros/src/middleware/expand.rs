use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;

use super::parse::{MiddlewareKind, SwordMiddlewareArgs};

pub fn expand_middleware_args(args: &MiddlewareKind) -> TokenStream {
    match args {
        MiddlewareKind::Sword(sword_args) => expand_sword_middleware(sword_args),
        MiddlewareKind::TowerLayer(expr) => expand_tower_layer_middleware(expr),
    }
}

fn expand_sword_middleware(args: &SwordMiddlewareArgs) -> TokenStream {
    let SwordMiddlewareArgs { path, config } = args;

    match config {
        Some(config) => quote! {
            ::sword::__internal::mw_with_state(
                app_state.clone(),
                |ctx: ::sword::web::Context, next: ::sword::web::Next| async move {
                    <#path>::handle(#config, ctx, next).await
                }
            )
        },
        None => quote! {
            ::sword::__internal::mw_with_state(
                app_state.clone(),
                |ctx: ::sword::web::Context, next: ::sword::web::Next| async move {
                    <#path>::handle(ctx, next).await
                }
            )
        },
    }
}

fn expand_tower_layer_middleware(expr: &Expr) -> TokenStream {
    quote! { #expr }
}
