use proc_macro_error::emit_error;
use proc_macro2::Ident;
use quote::ToTokens;
use regex::Regex;
use std::sync::LazyLock;
use syn::{Attribute, ImplItem, ImplItemFn, ItemImpl, LitStr, parse as syn_parse};

use crate::middleware::parse::MiddlewareKind;

const VALID_ROUTE_MACROS: &[&str; 6] =
    &["get", "post", "put", "patch", "delete", "middleware"];

pub const HTTP_METHODS: [&str; 5] = ["get", "post", "put", "delete", "patch"];

static PATH_KIND_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\/(?:[^\/{}]+|\{[^*{}][^{}]*\}|\{\*[^{}]+\})*(?:\/(?:[^\/{}]+|\{[^*{}][^{}]*\}|\{\*[^{}]+\}))*$").unwrap()
});

pub struct RouteInfo {
    pub method: String,
    pub path: String,
    pub handler_name: Ident,
    pub middlewares: Vec<MiddlewareKind>,
}

pub fn parse_routes(input: ItemImpl) -> Vec<RouteInfo> {
    let mut routes: Vec<RouteInfo> = vec![];

    for item in input.items.iter() {
        if !matches!(item, ImplItem::Fn(_)) {
            continue;
        }

        let Ok(handler) = syn_parse::<ImplItemFn>(item.to_token_stream().into())
        else {
            emit_error!(item, "Failed to parse function item");
            continue;
        };

        let mut route_path = String::new();
        let mut route_method = String::new();
        let mut middlewares: Vec<MiddlewareKind> = vec![];

        for attr in &handler.attrs {
            let Some(ident) = attr.path().get_ident() else {
                continue;
            };

            if !VALID_ROUTE_MACROS.contains(&ident.to_string().as_str()) {
                continue;
            }

            if ident == "middleware" {
                match attr.parse_args::<MiddlewareKind>() {
                    Ok(args) => middlewares.push(args),
                    Err(e) => emit_error!(attr, "Invalid middleware syntax: {}", e),
                }
            } else if HTTP_METHODS.contains(&ident.to_string().as_str()) {
                route_method = ident.to_string();

                match parse_route_path(attr) {
                    Ok(path) => route_path = path.value(),
                    Err(e) => emit_error!(attr, "{}", e),
                }
            }
        }

        routes.push(RouteInfo {
            method: route_method,
            path: route_path,
            handler_name: handler.sig.ident.clone(),
            middlewares,
        });
    }

    routes
}

pub fn parse_route_path(attr: &Attribute) -> Result<LitStr, String> {
    let Ok(path) = attr.parse_args::<LitStr>() else {
        return Err("Expected a string literal as path in HTTP method attribute, e.g., #[get(\"/path\")]".to_string());
    };

    let value = path.value();

    if !PATH_KIND_REGEX.is_match(&value) {
        return Err(format!("Invalid path format: {}.", value));
    }

    Ok(path)
}
