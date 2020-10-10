use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{custom_punctuation, parenthesized, parse_macro_input, Expr, Ident};

custom_punctuation!(SpreadDot, *.);

struct Spread {
    iterable: Expr,
    fields: Vec<Ident>,
}

impl Parse for Spread {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        parenthesized!(content in input);
        let iterable: Expr = content.parse()?;
        let mut fields: Vec<Ident> = vec![];
        while input.peek(SpreadDot) {
            input.parse::<SpreadDot>()?;
            fields.push(input.parse()?);
        }
        Ok(Spread { iterable, fields })
    }
}

// spr macro implementation.
pub fn spr_impl(input: TokenStream) -> TokenStream {
    let Spread { iterable, fields } = parse_macro_input!(input as Spread);

    let mut maps = vec![];
    for field in fields {
        maps.push(quote! {
            .map(|it| it.#field)
        });
    }

    TokenStream::from(quote! {
        #iterable.iter().cloned()
            #(#maps)*
            .collect()
    })
}
