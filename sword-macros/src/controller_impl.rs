use std::sync::OnceLock;

use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;
use regex::Regex;
use syn::{
    Attribute, Expr, ExprLit, Ident, ImplItem, ItemImpl, Lit, LitStr, Token, parse::Parse,
    parse_macro_input, spanned::Spanned,
};

static PATH_KIND_REGEX: OnceLock<Regex> = OnceLock::new();

fn path_kind_regex() -> &'static Regex {
    PATH_KIND_REGEX.get_or_init(|| {
        Regex::new(r"^\/(?:[^\/{}]+|\{[^*{}][^{}]*\}|\{\*[^{}]+\})*(?:\/(?:[^\/{}]+|\{[^*{}][^{}]*\}|\{\*[^{}]+\}))*$").unwrap()
    })
}

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
            for attr in function.attrs.iter() {
                let Some(http_method) = attr.path().get_ident() else {
                    emit_error!(
                        attr,
                        "Expected an HTTP method attribute like #[get(\"/path\")]"
                    );

                    return TokenStream::new();
                };

                let route_path = get_attr_http_route(attr);
                let method_name = &function.sig.ident;

                let route = match http_method.to_string().as_str() {
                    "get" => quote! {
                        .route(#route_path, ::axum::routing::get(#struct_self::#method_name))
                    },
                    "post" => quote! {
                        .route(#route_path, ::axum::routing::post(#struct_self::#method_name))
                    },
                    "put" => quote! {
                        .route(#route_path, ::axum::routing::put(#struct_self::#method_name))
                    },
                    "delete" => quote! {
                        .route(#route_path, ::axum::routing::delete(#struct_self::#method_name))
                    },
                    "patch" => quote! {
                        .route(#route_path, ::axum::routing::patch(#struct_self::#method_name))
                    },
                    _ => {
                        emit_error!("Unknown HTTP method: {}", http_method);
                        return TokenStream::new();
                    }
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

fn get_attr_http_route(attr: &Attribute) -> LitStr {
    let Ok(path) = attr.parse_args::<LitStr>() else {
        let message =
            "Expected a string literal as path in HTTP method attribute, e.g., #[get(\"/path\")]";

        emit_error!(attr, "{}", message);

        return LitStr::new("", attr.span());
    };

    let value = path.value();

    if !path_kind_regex().is_match(&value) {
        emit_error!(
            attr,
            "Invalid path format: `{}`. Expected a valid path like `/path` or `/path/param`",
            value
        );

        return LitStr::new("", path.span());
    }

    path
}
