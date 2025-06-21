mod controller;
mod middleware;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = attr;
    item
}

#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = attr;
    item
}

#[proc_macro_attribute]
pub fn put(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = attr;
    item
}

#[proc_macro_attribute]
pub fn delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = attr;
    item
}

#[proc_macro_attribute]
pub fn patch(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = attr;
    item
}

#[proc_macro_attribute]
pub fn controller_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    controller::expand_controller_impl(attr, item)
}

#[proc_macro_derive(Middleware)]
pub fn middleware_derive(item: TokenStream) -> TokenStream {
    middleware::expand_middleware_derive(item)
}

#[proc_macro_attribute]
pub fn middleware(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = attr;
    item
}
