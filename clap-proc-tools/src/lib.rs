//! # Procedural macro-tools for Clap
//!
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn ez_c_str(input: TokenStream) -> TokenStream {
    ::ffi_wrap::c_str(input.into()).into()
}

#[proc_macro_attribute]
pub fn extends(attrs: TokenStream, input: TokenStream) -> TokenStream {
    ::ffi_wrap::extension(attrs.into(), input.into()).into()
}

#[proc_macro]
pub fn entrypoint(input: TokenStream) -> TokenStream {
    ::ffi_wrap::entrypoint(input.into()).into()
}

#[proc_macro]
pub fn plugin_parameter(input: TokenStream) -> TokenStream {
    ::ffi_wrap::plugin_parameter(input.into()).into()
}
