use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;
use syn::{Attribute, Ident, ImplItem, ItemImpl, parse_macro_input};

use crate::{
    middleware::{expand_middleware_args, parse::MiddlewareKind},
    utils::{HTTP_METHODS, get_attr_http_route},
};

pub fn expand_controller_routes(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let struct_self = &input.self_ty;
    let mut routes = vec![];

    for item in input.items.iter() {
        if let ImplItem::Fn(function) = item {
            let mut middlewares: Vec<MiddlewareKind> = vec![];
            let mut http_meta: Option<(&Attribute, Ident)> = None;

            for attr in &function.attrs {
                if attr.path().is_ident("middleware") {
                    match attr.parse_args::<MiddlewareKind>() {
                        Ok(args) => middlewares.push(args),
                        Err(e) => {
                            emit_error!(attr, "Invalid middleware syntax: {}", e);
                            continue;
                        }
                    }
                } else if let Some(ident) = attr.path().get_ident()
                    && HTTP_METHODS.contains(&ident.to_string().as_str())
                {
                    http_meta = Some((attr, ident.clone()));
                }
            }

            if let Some((http_attr, http_ident)) = http_meta {
                let route_path = get_attr_http_route(http_attr);
                let method_name = &function.sig.ident;

                let routing_fn = match http_ident.to_string().as_str() {
                    "get" => quote! { axum_get_fn },
                    "post" => quote! { axum_post_fn },
                    "put" => quote! { axum_put_fn },
                    "patch" => quote! { axum_patch_fn },
                    "delete" => quote! { axum_delete_fn },
                    _ => {
                        emit_error!(
                            http_attr,
                            "Unsupported HTTP method: {}",
                            http_ident
                        );
                        continue;
                    }
                };

                let mut handler = quote! {
                    ::sword::__internal::#routing_fn({
                        let controller_clone = std::sync::Arc::clone(&controller);
                        move |ctx: ::sword::web::Context| {
                            let controller_result = controller_clone.clone();

                            async move {
                                use sword::__internal::IntoResponse;

                                match controller_result.as_ref() {
                                    Ok(controller) => {
                                        controller.#method_name(ctx).await.into_response()
                                    },
                                    Err(e) => ::sword::web::HttpResponse::InternalServerError()
                                        .message(format!("Controller build error: {e}"))
                                        .into_response(),
                                }
                            }
                        }
                    })
                };

                for mw in middlewares.iter().rev() {
                    let mw_tokens = expand_middleware_args(mw);
                    handler = quote! {
                        #handler.layer(#mw_tokens)
                    };
                }

                let route = quote! {
                    .route(#route_path, #handler)
                };

                routes.push(route);
            }
        }
    }

    let expanded = quote! {
        #input

        impl ::sword::web::Controller for #struct_self
            where Self: ::sword::web::ControllerBuilder
        {
            fn router(app_state: ::sword::core::State) -> ::sword::__internal::AxumRouter {
                let controller = std::sync::Arc::new(Self::build(app_state.clone()));

                let base_router = ::sword::__internal::AxumRouter::new()
                    #(#routes)*
                    .with_state(app_state.clone());

                let base_path = #struct_self::base_path();
                let router_with_global_mw = #struct_self::apply_controller_middlewares(base_router, app_state.clone());

                if base_path == "/" {
                    router_with_global_mw
                } else {
                    ::sword::__internal::AxumRouter::new()
                        .nest(base_path, router_with_global_mw)
                }
            }
        }
    };

    TokenStream::from(expanded)
}
