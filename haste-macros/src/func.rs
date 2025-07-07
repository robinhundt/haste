use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Error, ItemFn, ItemMod, Stmt, parse::Parse};

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
    let Args {
        args,
        runtime,
        throughput,
    } = args;
    let BenchFunc { mut func } = bench_func;

    if args.is_some() && func.sig.inputs.len() != 1 {
        return Error::new_spanned(
            func,
            "When args are supplied, the benchmarked function must have exactly one parameter.",
        )
        .into_compile_error();
    }
    let ident = func.sig.ident.clone();
    let ident_str = ident.to_string();
    let is_async = func.sig.asyncness.is_some();

    let mut runtime_tokens = None;
    if is_async {
        runtime_tokens = Some(match runtime {
            Some(rt_expr) => quote! {
                let rt = #rt_expr;
            },
            None => {
                quote! {
                    let rt = ::haste::__private::tokio::runtime::Runtime::new().unwrap();
                }
            }
        })
    }

    let mut bench_call = quote! {b};
    if let Some(throughput) = throughput {
        bench_call = quote! {
            #bench_call.with_throughput(#throughput)
        };
    }
    let bench_arg = args.as_ref().map(|_| {
        quote! {
            ::std::hint::black_box(arg)
        }
    });

    if is_async {
        bench_call = quote! {
            #bench_call.bench_async(label, &rt, || {
                super::#ident(#bench_arg)
            })
        };
    } else {
        bench_call = quote! {
            #bench_call.bench(label, || {
                super::#ident(#bench_arg)
            });
        }
    }

    let bench_body = match (func.sig.inputs.len(), args) {
        (0, None) => Some(quote! {
            #runtime_tokens
            let label = ::haste::Label::new(#ident_str);
            #bench_call
        }),
        (1, None) => None,
        (1, Some(args)) => Some(quote! {
            #runtime_tokens
            for arg in [#args] {
                let label = ::haste::Label::new(#ident_str).with_part(&arg);
                #bench_call
            }
        }),
        (count, _) => {
            return Error::new_spanned(func, format!("Unsupported number of parameters: {count}"))
                .into_compile_error();
        }
    };

    let bench_module = bench_body.map(|body| {
        quote! {
            #[doc(hidden)]
            mod __haste_bench {
                use super::*;

                #[::haste::__private::distributed_slice(haste::__private::BENCHMARKS)]
                #[linkme(crate = haste::__private::linkme)]
                fn #ident(mut b: ::haste::Haste) {
                    #body
                }
            }
        }
    });

    if let Some(bench_module) = bench_module {
        let bench_module: ItemMod = syn::parse2(bench_module)
            .expect("error in macro expansion. bench_module can't be parsed as ItemMod");
        func.block
            .stmts
            .push(Stmt::Item(syn::Item::Mod(bench_module)));
        func.to_token_stream()
    } else {
        quote! {
            #[::haste::__private::distributed_slice(haste::__private::BENCHMARKS)]
            #[linkme(crate = haste::__private::linkme)]
            #func
        }
    }
}
