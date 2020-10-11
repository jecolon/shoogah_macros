//! shoogah_macros is a helper that provides the macros for the `shoogah` crate.

mod cxp;
mod hml;
mod sin;
mod spr;
mod suf;

#[macro_use]
extern crate lazy_static;
use cxp::{cxp_impl, ela_impl, elv_impl};
use hml::hml_impl;
use proc_macro::TokenStream;
use sin::sin_impl;
use spr::spr_impl;
use suf::suf_impl;

/// Define a `std::collections::HashMap` via a simple literal.
///
/// Refer to the `shoogah` crate documentation for details.
#[proc_macro]
pub fn hml(input: TokenStream) -> TokenStream {
    hml_impl(input)
}

/// Express an if/else in a shorthand manner. This is sometimes called the
/// *ternary* operator in other languages.
///
/// Refer to the `shoogah` crate documentation for details.
#[proc_macro]
pub fn cxp(input: TokenStream) -> TokenStream {
    cxp_impl(input)
}

/// When the main result of an if/else is the same as the tested condition,
/// Elvis (elv!) is here to help.
///
/// Refer to the `shoogah` crate documentation for details.
#[proc_macro]
pub fn elv(input: TokenStream) -> TokenStream {
    elv_impl(input)
}

/// When the assigned-to variable is the condition being tested, Elvis assign
/// (ela!) can help even more.
///
/// Refer to the `shoogah` crate documentation for details.
#[proc_macro]
pub fn ela(input: TokenStream) -> TokenStream {
    ela_impl(input)
}

/// Incrementing or decrementing by 1.
///
/// Refer to the `shoogah` crate documentation for details.
#[proc_macro]
pub fn suf(input: TokenStream) -> TokenStream {
    suf_impl(input)
}

/// # Collect common field values from an `Iterator`.
///
/// Refer to the `shoogah` crate documentation for details.
#[proc_macro]
pub fn spr(input: TokenStream) -> TokenStream {
    spr_impl(input)
}

/// # String interpolation.
///
/// Refer to the `shoogah` crate documentation for details.
#[proc_macro]
pub fn sin(input: TokenStream) -> TokenStream {
    sin_impl(input)
}
