use proc_macro_error::emit_error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

use crate::{
    controller::routes::parsing::RouteInfo, middleware::expand_middleware_args,
};

pub fn generate_controller_routes(
    struct_self: &Type,
    routes: Vec<RouteInfo>,
) -> TokenStream {
    let mut handlers = vec![];

    for route in routes.iter() {
        let mut routing_function = quote! {};

        let route_method = &route.method;
        let route_path = &route.path;
        let handler_name = &route.handler_name;

        match route_method.as_str() {
            "get" => routing_function = quote! { axum_get_fn },
            "post" => routing_function = quote! { axum_post_fn },
            "put" => routing_function = quote! { axum_put_fn },
            "patch" => routing_function = quote! { axum_patch_fn },
            "delete" => routing_function = quote! { axum_delete_fn },
            _ => emit_error!("Unsupported HTTP method: {}", route.method),
        };

        let mut handler = quote! {
            ::sword::__internal::#routing_function({
                let controller_build = std::sync::Arc::clone(&controller);

                move |ctx: ::sword::web::Context| {
                    let controller_build = controller_build.clone();

                    async move {
                        use ::sword::__internal::IntoResponse;

                        match controller_build.as_ref() {
                            Ok(controller) => controller.#handler_name(ctx).await.into_response(),
                            Err(err) => ::sword::web::HttpResponse::InternalServerError()
                                .message(format!("Controller build error: {err}"))
                                .into_response(),
                        }
                    }
                }
            })
        };

        for middleware in route.middlewares.iter().rev() {
            let generated_middleware = expand_middleware_args(middleware);

            handler = quote! {
                #handler.layer(#generated_middleware)
            };
        }

        handlers.push(quote! {
            .route(#route_path, #handler)
        });
    }

    quote! {
        impl ::sword::web::Controller for #struct_self
        where
            Self: ::sword::web::ControllerBuilder
        {
            fn router(state: ::sword::core::State) -> ::sword::__internal::AxumRouter {
                let controller = std::sync::Arc::new(Self::build(state.clone()));

                let base_router = ::sword::__internal::AxumRouter::new()
                    #(#handlers)*
                    .with_state(state.clone());


                let base_path = #struct_self::base_path();
                let router = #struct_self::apply_controller_middlewares(base_router, state);

                match base_path {
                    "/" => router,
                    _ => ::sword::__internal::AxumRouter::new()
                        .nest(base_path, router),
                }
            }
        }
    }
}
