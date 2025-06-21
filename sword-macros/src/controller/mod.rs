mod path_utils;

use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;
use syn::{
    Attribute, Expr, ExprLit, Ident, ImplItem, ItemImpl, Lit, Path, Token, parse::Parse,
    parse_macro_input, punctuated::Punctuated, spanned::Spanned,
};

use crate::controller::path_utils::HTTP_METHODS;

struct ControllerImplArgs {
    prefix: String,
}

impl Parse for ControllerImplArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        if ident != "prefix" {
            return Err(syn::Error::new(ident.span(), "Expected 'prefix'"));
        }

        let _: Token![=] = input.parse()?;
        let expr: Expr = input.parse()?;

        if let Expr::Lit(ExprLit {
            lit: Lit::Str(lit_str),
            ..
        }) = expr
        {
            Ok(ControllerImplArgs {
                prefix: lit_str.value(),
            })
        } else {
            Err(syn::Error::new(expr.span(), "Expected string literal"))
        }
    }
}

pub fn expand_controller_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let args = parse_macro_input!(attr as ControllerImplArgs);

    let router_prefix_str = args.prefix;

    let struct_self = &input.self_ty;
    let mut routes = vec![];

    for item in input.items.iter() {
        if let ImplItem::Fn(function) = item {
            let mut middlewares = Vec::new();
            let mut http_meta: Option<(&Attribute, Ident)> = None;

            for attr in &function.attrs {
                if attr.path().is_ident("middleware") {
                    match attr.parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated) {
                        Ok(paths) => middlewares = paths.into_iter().collect(),
                        Err(err) => {
                            emit_error!(
                                attr,
                                "Expected a comma-separated list of middleware types: {}",
                                err
                            );

                            return TokenStream::new();
                        }
                    }
                } else if let Some(ident) = attr.path().get_ident() {
                    if HTTP_METHODS.contains(&ident.to_string().as_str()) {
                        http_meta = Some((attr, ident.clone()));
                    }
                }
            }

            if let Some((http_attr, http_ident)) = http_meta {
                let route_path = path_utils::get_attr_http_route(http_attr);
                let method_name = &function.sig.ident;

                let mut handler = quote! {
                    ::axum::routing::#http_ident(#struct_self::#method_name)
                };

                for mw in middlewares.iter().rev() {
                    handler = quote! { #handler.layer(::axum::middleware::from_fn(#mw::middleware_handle)) };
                }

                let route = quote! {
                    .route(#route_path, #handler)
                };

                routes.push(route);
            }
        }
    }

    let base_router = quote! {
        ::axum::Router::new()
            #(#routes)*
    };

    let expanded = quote! {
        #input

        impl ::sword::routing::RouterProvider for #struct_self {
            fn router() -> ::axum::Router {
                ::axum::Router::new()
                    .nest(#router_prefix_str, #base_router)
            }
        }
    };

    TokenStream::from(expanded)
}
