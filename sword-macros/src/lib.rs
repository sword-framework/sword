mod controller;
mod utils;

use crate::controller::implementation;
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
pub fn controller(attr: TokenStream, item: TokenStream) -> TokenStream {
    controller::expand_controller(attr, item)
}

#[proc_macro_attribute]
pub fn controller_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    implementation::expand_controller_impl(attr, item)
}

#[proc_macro_attribute]
pub fn middleware(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = attr;
    item
}
