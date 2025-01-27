//! Intermediate representation
//!
//!

use proc_macro2::TokenStream as TokenStream2;

pub mod c_str;
pub mod ext;
pub mod implementation;
pub mod plugin;
pub mod transparent_wrapper;

pub fn parse_c_str(input: TokenStream2) -> TokenStream2 {
    c_str::parse(input)
}

pub fn parse_plugin_entrypoint(input: TokenStream2) -> TokenStream2 {
    plugin::entrypoint::parse(input)
}

pub fn parse_extension(attrs: TokenStream2, input: TokenStream2) -> TokenStream2 {
    ext::parse(attrs, input)
}

pub fn parse_plugin_parameter(input: TokenStream2) -> TokenStream2 {
    plugin::parameter::parse(input)
}
