//! Intermediate representation
//!
//!

pub mod c_str;
pub mod clap_extension;
pub mod implementation;
pub mod transparent_wrapper;

use proc_macro2::TokenStream as TokenStream2;

pub fn parse_c_str(input: TokenStream2) -> TokenStream2 {
    c_str::parse(input)
}

pub fn parse_clap_extension(attrs: TokenStream2, input: TokenStream2) -> TokenStream2 {
    todo!()
}
