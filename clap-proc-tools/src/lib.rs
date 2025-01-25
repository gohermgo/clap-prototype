//! # Procedural macro-tools for Clap
//!
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn ez_string(input: TokenStream) -> TokenStream {
    ::ffi_wrap::ez_string(input.into()).into()
}
