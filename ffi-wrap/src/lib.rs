use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn c_str(input: TokenStream2) -> TokenStream2 {
    let item = clap_proc_ir::c_str::parse(input);
    quote! { #item }
}

pub fn entrypoint(input: TokenStream2) -> TokenStream2 {
    let item = clap_proc_ir::parse_plugin_entrypoint(input);
    quote! {#item}
}

pub fn extension(attrs: TokenStream2, input: TokenStream2) -> TokenStream2 {
    let item = clap_proc_ir::parse_extension(attrs, input);
    quote! { #item }
}

pub fn plugin_parameter(input: TokenStream2) -> TokenStream2 {
    let item = clap_proc_ir::parse_plugin_parameter(input);
    quote! {#item}
}
