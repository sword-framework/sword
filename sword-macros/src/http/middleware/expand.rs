use proc_macro2::TokenStream;
use quote::quote;

use crate::http::middleware::MiddlewareArgs;

pub fn expand_middleware_args(args: &MiddlewareArgs) -> TokenStream {
    let MiddlewareArgs { path, config } = args;

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
