use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, ItemFn, parse::Parse, spanned::Spanned};

use crate::Args;

pub struct BenchFunc {
    func: ItemFn,
}

impl Parse for BenchFunc {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let func = input.parse()?;
        let bench_func = BenchFunc { func };

        Ok(bench_func)
    }
}

pub(crate) fn expand(args: Args, bench_func: BenchFunc) -> TokenStream {
    let Args { args } = args;
    let BenchFunc { func } = bench_func;

    if args.is_some() && func.sig.inputs.len() != 1 {
        return Error::new(
            func.span(),
            "When args are supplied, the benchmarked function must have exactly one parameter.",
        )
        .into_compile_error();
    }
    let ident = func.sig.ident.clone();
    let ident_str = ident.to_string();

    let benching = if let Some(args) = args {
        quote! {
            for arg in [#args] {
                let label = ::haste::Label::new(#ident_str).with_part(&arg);
                b.bench_function(label, || {
                    #ident(::std::hint::black_box(arg))
                })
            }
        }
    } else {
        quote! {
            b.bench_function(#ident_str, || {
                #ident()
            })
        }
    };

    quote! {
        #[::haste::__private::distributed_slice(haste::__private::BENCHMARKS)]
        #[linkme(crate = haste::__private::linkme)]
        fn #ident(b: &mut ::haste::Bencher) {
            #func
            #benching
        }
    }
}
