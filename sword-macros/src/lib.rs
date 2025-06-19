mod controller;
mod controller_impl;
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
    controller_impl::expand_controller_impl(attr, item)
}

#[proc_macro_derive(Controller)]
pub fn controller(item: TokenStream) -> TokenStream {
    controller::expand_controller_derive(TokenStream::new(), item)
}
