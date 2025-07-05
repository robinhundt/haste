use syn::{Error, parse::Parse};

pub(crate) struct Args {}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            Ok(Args {})
        } else {
            Err(Error::new(input.span(), "No arguments allowed"))
        }
    }
}
