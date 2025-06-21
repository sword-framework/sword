use proc_macro::TokenStream;

use quote::quote;
use syn::{ItemStruct, parse_macro_input};

pub fn expand_middleware_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;

    let expanded = quote! {
        impl #struct_name {
            pub async fn middleware_handle(
                req: ::axum::extract::Request,
                next: ::axum::middleware::Next,
            ) -> ::axum::response::Response {
                let sword_req = ::sword::http::Request::from_axum_request(req);
                let sword_next = ::sword::middleware::NextFunction::new(next);
                match Self::handle(sword_req, sword_next).await {
                    Ok(response) => response,
                    Err(err) => {
                        use ::axum::response::IntoResponse;
                        err.into_response()
                    }
                }
            }
        }
    };

    TokenStream::from(expanded)
}
