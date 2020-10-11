use proc_macro::TokenStream;
use quote::quote;
use regex::Regex;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, parse_str, Expr, LitStr};

// Interpolation holds the pieces of a string interpolation.
struct Interpolation {
    fmt_str: String,
    expressions: Vec<Expr>,
}

impl Parse for Interpolation {
    fn parse(input: ParseStream) -> Result<Self> {
        lazy_static! {
            static ref SIN_RE: Regex = Regex::new(r"\$\{([^}]+)\}").unwrap();
        };
        let literal: LitStr = input.parse()?;
        let fmt_str = literal.value();
        let mut expressions: Vec<Expr> = vec![];
        if !SIN_RE.is_match(&fmt_str) {
            return Ok(Interpolation {
                fmt_str,
                expressions,
            });
        }
        for capture in SIN_RE.captures_iter(&fmt_str) {
            expressions.push(parse_str(&capture[1])?);
        }
        let fmt_str = SIN_RE.replace_all(&fmt_str, "{}").to_string();

        Ok(Interpolation {
            fmt_str,
            expressions,
        })
    }
}

// sin macro implementation.
pub fn sin_impl(input: TokenStream) -> TokenStream {
    let Interpolation {
        fmt_str,
        expressions,
    } = parse_macro_input!(input as Interpolation);
    if expressions.is_empty() {
        return TokenStream::from(quote! { #fmt_str });
    }

    TokenStream::from(quote! {
        format!(#fmt_str, #(#expressions),*)
    })
}
