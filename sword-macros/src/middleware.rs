use proc_macro::TokenStream;

use quote::quote;
use syn::{ItemStruct, parse_macro_input};

pub fn expand_middleware_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;

    let expanded = quote! {
        impl #struct_name {
            pub async fn middleware_handle(
                State(state): ::axum::extract::State<::sword::application::AppState>,
                req: ::axum::extract::Request,
                next: ::axum::middleware::Next,
            ) -> ::axum::response::Response {
                use ::axum::extract::FromRequest;
                use ::axum::response::IntoResponse;

                let sword_req = match ::sword::http::Request::from_request(req, &state).await {
                    Ok(req) => req,
                    Err(response) => return response.into_response(),
                };

                let sword_next = ::sword::middleware::NextFunction::new(next);
                match Self::handle(sword_req, sword_next).await {
                    Ok(response) => response,
                    Err(err) => err.into_response(),
                }
            }
        }
    };

    TokenStream::from(expanded)
}
