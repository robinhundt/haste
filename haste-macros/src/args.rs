use syn::{Error, Expr, Ident, Token, bracketed, parse::Parse, punctuated::Punctuated};

#[derive(Default)]
pub(crate) struct Args {
    pub(crate) args: Option<Punctuated<Expr, Token![,]>>,
    pub(crate) runtime: Option<Expr>,
    pub(crate) throughput: Option<Expr>,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Args::default());
        }
        let mut args = None;
        let mut runtime = None;
        let mut throughput = None;

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            match ident.to_string().as_str() {
                "args" => {
                    if args.is_some() {
                        return Err(Error::new_spanned(ident, "duplicate args"));
                    }
                    input.parse::<Token![=]>()?;
                    let content;
                    bracketed!(content in input);
                    args = Some(Punctuated::parse_terminated(&content)?);
                }
                "runtime" => {
                    if runtime.is_some() {
                        return Err(Error::new_spanned(ident, "duplicate runtime"));
                    }
                    input.parse::<Token![=]>()?;
                    runtime = Some(input.parse()?);
                }
                "throughput" => {
                    if throughput.is_some() {
                        return Err(Error::new_spanned(ident, "duplicate throughput"));
                    }
                    input.parse::<Token![=]>()?;
                    throughput = Some(input.parse()?);
                }
                _ => {
                    return Err(Error::new_spanned(
                        &ident,
                        format!("unknown argument {ident}"),
                    ));
                }
            }
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Args {
            args,
            runtime,
            throughput,
        })
    }
}
