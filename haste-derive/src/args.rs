use syn::{Error, Expr, Ident, Token, bracketed, parse::Parse, punctuated::Punctuated};

#[derive(Default)]
pub(crate) struct Args {
    pub(crate) args: Option<Punctuated<Expr, Token![,]>>,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item = input.cursor().token_stream();
        if input.is_empty() {
            return Ok(Args::default());
        }
        if input.parse::<Ident>()? != "args" {
            return Err(Error::new_spanned(
                item,
                "correct usage: #[haste(args = [..])]",
            ));
        }
        input.parse::<Token![=]>()?;
        let content;
        bracketed!(content in input);
        let args: Punctuated<Expr, Token![,]> = Punctuated::parse_terminated(&content)?;

        Ok(Args { args: Some(args) })
    }
}
