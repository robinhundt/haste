use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::{
    args::Args,
    func::{BenchFunc, expand},
};

mod args;
mod func;

#[proc_macro_attribute]
pub fn haste(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as Args);
    let bench_func = parse_macro_input!(input as BenchFunc);
    let expanded = expand(args, bench_func);
    TokenStream::from(expanded)
}
