use std::sync::OnceLock;

use proc_macro_error::emit_error;
use regex::Regex;
use syn::{Attribute, LitStr, spanned::Spanned};

pub const HTTP_METHODS: [&str; 5] = ["get", "post", "put", "delete", "patch"];

static PATH_KIND_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn path_kind_regex() -> &'static Regex {
    PATH_KIND_REGEX.get_or_init(|| {
        Regex::new(r"^\/(?:[^\/{}]+|\{[^*{}][^{}]*\}|\{\*[^{}]+\})*(?:\/(?:[^\/{}]+|\{[^*{}][^{}]*\}|\{\*[^{}]+\}))*$").unwrap()
    })
}

pub fn get_attr_http_route(attr: &Attribute) -> LitStr {
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
