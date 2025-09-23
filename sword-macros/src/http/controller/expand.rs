use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;
use syn::{ItemStruct, LitStr, parse_macro_input};

use crate::http::{
    controller::fields::collect_controller_fields,
    middleware::{expand_middleware_args, parse::MiddlewareKind},
};

pub fn expand_controller(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let args = parse_macro_input!(attr as LitStr);

    let router_prefix_str = args.value();
    let struct_name = &input.ident;

    let mut route_middlewares = vec![];

    for attr in &input.attrs {
        if attr.path().is_ident("middleware") {
            match attr.parse_args::<MiddlewareKind>() {
                Ok(args) => route_middlewares.push(expand_middleware_args(&args)),
                Err(e) => emit_error!("Failed to parse middleware arguments: {}", e),
            }
        }
    }

    let self_fields = collect_controller_fields(&input);

    let field_extractions = self_fields.iter().map(|(field_name, field_type)| {
        quote! {
            let #field_name = state.get::<#field_type>().map_err(|e| {
                ControllerError::StateExtractionError(format!(
                    "Failed to extract {} from state: {}",
                    stringify!(#field_type),
                    e
                ))
            })?;
        }
    });

    let field_assignments = self_fields.iter().map(|(field_name, _)| {
        quote! { #field_name }
    });

    let expanded_builder = quote! {
        impl ::sword::web::ControllerBuilder for #struct_name {
            fn build(state: ::sword::core::State) -> Result<Self, ::sword::web::ControllerError> {
                #(#field_extractions)*

                Ok(Self {
                    #(#field_assignments),*
                })
            }
        }
    };

    let expanded = quote! {
        #input

        impl #struct_name {
            fn prefix() -> &'static str {
                #router_prefix_str
            }

            fn apply_global_middlewares(router: ::sword::__internal::AxumRouter, app_state: ::sword::core::State) -> ::sword::__internal::AxumRouter {
                let mut result = router;

                #(
                    result = result.layer(#route_middlewares);
                )*

                result
            }
        }

        #expanded_builder

    };

    TokenStream::from(expanded)
}
