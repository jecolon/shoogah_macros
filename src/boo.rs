use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr};

// boo macro implementation.
pub fn boo_impl(input: TokenStream) -> TokenStream {
    let expr: Expr = parse_macro_input!(input as Expr);

    TokenStream::from(quote! {{
        use shoogah::AsBool;
        (#expr).as_bool()
    }})
}
