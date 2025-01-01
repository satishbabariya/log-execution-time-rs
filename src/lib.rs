extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// A procedural macro to log the execution time of a function.
#[proc_macro_attribute]
pub fn log_execution_time(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let func_name = &input.sig.ident;
    let block = &input.block;
    let visibility = &input.vis;
    let sig = &input.sig;

    let expanded = quote! {
        #visibility #sig {
            let start = std::time::Instant::now();
            let result = (|| #block)();
            let duration = start.elapsed();
            log::info!("Execution time of `{}`: {:.2?}", stringify!(#func_name), duration);
            result
        }
    };

    TokenStream::from(expanded)
}
