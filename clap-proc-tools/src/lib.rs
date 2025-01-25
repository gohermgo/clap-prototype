//! # Procedural macro-tools for Clap
//!
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn ez_c_str(input: TokenStream) -> TokenStream {
    ::ffi_wrap::c_str(input.into()).into()
}
