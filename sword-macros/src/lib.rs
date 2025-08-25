mod config;
mod controller;
mod utils;

use crate::controller::implementation;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote};

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

#[deprecated(since = "0.1.5", note = "Use `#[routes]` instead")]
#[proc_macro_attribute]
pub fn controller_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    implementation::expand_controller_impl(attr, item)
}

#[proc_macro_attribute]
pub fn routes(attr: TokenStream, item: TokenStream) -> TokenStream {
    implementation::expand_controller_impl(attr, item)
}

#[proc_macro_attribute]
pub fn middleware(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = attr;
    item
}

#[proc_macro_attribute]
pub fn config(attr: TokenStream, item: TokenStream) -> TokenStream {
    config::expand_config_struct(attr, item)
}

/// ### This is just a re-export of `tokio::main` to simplify the initial setup of
/// ### Sword, you can use your own version of tokio adding it to your
/// ### `Cargo.toml`, we are providing this initial base by default
///
/// ---
///
/// Marks async function to be executed by the selected runtime. This macro
/// helps set up a `Runtime` without requiring the user to use
/// [Runtime](../tokio/runtime/struct.Runtime.html) or
/// [Builder](../tokio/runtime/struct.Builder.html) directly.
///
/// Note: This macro is designed to be simplistic and targets applications that
/// do not require a complex setup. If the provided functionality is not
/// sufficient, you may be interested in using
/// [Builder](../tokio/runtime/struct.Builder.html), which provides a more
/// powerful interface.
///
/// Note: This macro can be used on any function and not just the `main`
/// function. Using it on a non-main function makes the function behave as if it
/// was synchronous by starting a new runtime each time it is called. If the
/// function is called often, it is preferable to create the runtime using the
/// runtime builder so the runtime can be reused across calls.
///
/// # Non-worker async function
///
/// Note that the async function marked with this macro does not run as a
/// worker. The expectation is that other tasks are spawned by the function here.
/// Awaiting on other futures from the function provided here will not
/// perform as fast as those spawned as workers.
///
/// # Multi-threaded runtime
///
/// To use the multi-threaded runtime, the macro can be configured using
///
/// ```rust,ignore
/// #[tokio::main(flavor = "multi_thread", worker_threads = 10)]
/// # async fn main() {}
/// ```
///
/// The `worker_threads` option configures the number of worker threads, and
/// defaults to the number of cpus on the system. This is the default flavor.
///
/// Note: The multi-threaded runtime requires the `rt-multi-thread` feature
/// flag.
///
/// # Current thread runtime
///
/// To use the single-threaded runtime known as the `current_thread` runtime,
/// the macro can be configured using
///
/// ```rust,ignore
/// #[tokio::main(flavor = "current_thread")]
/// # async fn main() {}
/// ```
///
/// ## Function arguments:
///
/// Arguments are allowed for any functions aside from `main` which is special
///
/// ## Usage
///
/// ### Using the multi-thread runtime
///
/// ```ignore
/// #[tokio::main]
/// async fn main() {
///     println!("Hello world");
/// }
/// ```
///
/// Equivalent code not using `#[tokio::main]`
///
/// ```ignore
/// fn main() {
///     tokio::runtime::Builder::new_multi_thread()
///         .enable_all()
///         .build()
///         .unwrap()
///         .block_on(async {
///             println!("Hello world");
///         })
/// }
/// ```
///
/// ### Using current thread runtime
///
/// The basic scheduler is single-threaded.
///
/// ```ignore
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() {
///     println!("Hello world");
/// }
/// ```
///
/// Equivalent code not using `#[tokio::main]`
///
/// ```ignore
/// fn main() {
///     tokio::runtime::Builder::new_current_thread()
///         .enable_all()
///         .build()
///         .unwrap()
///         .block_on(async {
///             println!("Hello world");
///         })
/// }
/// ```
///
/// ### Set number of worker threads
///
/// ```ignore
/// #[tokio::main(worker_threads = 2)]
/// async fn main() {
///     println!("Hello world");
/// }
/// ```
///
/// Equivalent code not using `#[tokio::main]`
///
/// ```ignore
/// fn main() {
///     tokio::runtime::Builder::new_multi_thread()
///         .worker_threads(2)
///         .enable_all()
///         .build()
///         .unwrap()
///         .block_on(async {
///             println!("Hello world");
///         })
/// }
/// ```
///
/// ### Configure the runtime to start with time paused
///
/// ```ignore
/// #[tokio::main(flavor = "current_thread", start_paused = true)]
/// async fn main() {
///     println!("Hello world");
/// }
/// ```
///
/// Equivalent code not using `#[tokio::main]`
///
/// ```ignore
/// fn main() {
///     tokio::runtime::Builder::new_current_thread()
///         .enable_all()
///         .start_paused(true)
///         .build()
///         .unwrap()
///         .block_on(async {
///             println!("Hello world");
///         })
/// }
/// ```
///
/// Note that `start_paused` requires the `test-util` feature to be enabled.
///
/// ### Rename package
///
/// ```ignore
/// use tokio as tokio1;
///
/// #[tokio1::main(crate = "tokio1")]
/// async fn main() {
///     println!("Hello world");
/// }
/// ```
///
/// Equivalent code not using `#[tokio::main]`
///
/// ```ignore
/// use tokio as tokio1;
///
/// fn main() {
///     tokio1::runtime::Builder::new_multi_thread()
///         .enable_all()
///         .build()
///         .unwrap()
///         .block_on(async {
///             println!("Hello world");
///         })
/// }
/// ```
///
/// ### Configure unhandled panic behavior
///
/// Available options are `shutdown_runtime` and `ignore`. For more details, see
/// [`Builder::unhandled_panic`].
///
/// This option is only compatible with the `current_thread` runtime.
///
/// ```no_run, ignore
/// # #![allow(unknown_lints, unexpected_cfgs)]
/// #[cfg(tokio_unstable)]
/// #[tokio::main(flavor = "current_thread", unhandled_panic = "shutdown_runtime")]
/// async fn main() {
///     let _ = tokio::spawn(async {
///         panic!("This panic will shutdown the runtime.");
///     }).await;
/// }
/// # #[cfg(not(tokio_unstable))]
/// # fn main() { }
/// ```
///
/// Equivalent code not using `#[tokio::main]`
///
/// ```no_run, ignore
/// # #![allow(unknown_lints, unexpected_cfgs)]
/// #[cfg(tokio_unstable)]
/// fn main() {
///     tokio::runtime::Builder::new_current_thread()
///         .enable_all()
///         .unhandled_panic(UnhandledPanic::ShutdownRuntime)
///         .build()
///         .unwrap()
///         .block_on(async {
///             let _ = tokio::spawn(async {
///                 panic!("This panic will shutdown the runtime.");
///             }).await;
///         })
/// }
/// # #[cfg(not(tokio_unstable))]
/// # fn main() { }
/// ```
///
/// **Note**: This option depends on Tokio's [unstable API][unstable]. See [the
/// documentation on unstable features][unstable] for details on how to enable
/// Tokio's unstable features.
///
/// [`Builder::unhandled_panic`]: ../tokio/runtime/struct.Builder.html#method.unhandled_panic
/// [unstable]: ../tokio/index.html#unstable-features
#[proc_macro_attribute]
pub fn main(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);

    println!("{}", quote! { #input });
    let mut fn_body = input.block.clone();
    let fn_attrs = input.attrs.clone();
    let fn_vis = input.vis.clone();
    let _fn_sig = input.sig.clone();

    let output = if cfg!(feature = "hot_reloading") {
        quote! {
            async fn __internal_main() -> Result<(), Box<dyn std::error::Error>> {
                #fn_body

                Ok::<(), Box<dyn std::error::Error>>(())
            }

            #(#fn_attrs)*
            #fn_vis fn main() {
                ::tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .expect("Failed building the Runtime")
                    .block_on(dioxus_devtools::serve_subsecond(__internal_main));
            }
        }
    } else {
        fn_body
            .stmts
            .push(parse_quote!({ Ok::<(), Box<dyn std::error::Error>>(()) }));

        quote! {
            #(#fn_attrs)*
            #fn_vis fn main() {
                ::tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .expect("Failed building the Runtime")
                    .block_on( async #fn_body );
            }
        }
    };

    output.into()
}
