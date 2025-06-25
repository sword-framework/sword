use proc_macro::TokenStream;

use quote::quote;
use syn::{ItemStruct, parse_macro_input};

pub fn expand_middleware_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;

    let expanded = quote! {
        impl #struct_name {
            pub async fn middleware_handle(
                ::sword::__private::State(state): ::sword::__private::State<::sword::application::AppState>,
                req: ::sword::__private::AxumRequest,
                next: ::sword::__private::AxumNext,
            ) -> ::sword::__private::AxumResponse {
                use ::sword::__private::FromRequest;
                use ::sword::__private::IntoResponse;
                use ::sword::middleware::MiddlewareHandler;

                let sword_req = match ::sword::http::Request::from_request(req, &state).await {
                    Ok(req) => req,
                    Err(response) => return response.into_response(),
                };

                let sword_next = ::sword::middleware::Next::new(next);
                match #struct_name::handle(sword_req, sword_next).await {
                    Ok(response) => response,
                    Err(err) => err.into_response(),
                }
            }
        }
    };

    TokenStream::from(expanded)
}
