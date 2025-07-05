use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Error, Ident, ItemFn, Token, Visibility, parse::Parse};

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

pub(crate) fn expand(bench_func: BenchFunc) -> TokenStream {
    let BenchFunc { func } = bench_func;
    let ident = func.sig.ident.clone();
    let ident_str = ident.to_string();
    quote! {
        #[::haste::__private::distributed_slice(haste::__private::BENCHMARKS)]
        #[linkme(crate = haste::__private::linkme)]
        fn #ident(b: &mut ::haste::Bencher) {
            #func
            b.bench_function(#ident_str, || {
                #ident()
            })
        }
    }
}
