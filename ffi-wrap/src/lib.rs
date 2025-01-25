use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub mod ir;

pub fn ez_c_str(input: TokenStream2) -> TokenStream2 {
    let item: syn::Result<ir::EzCStr> = syn::parse2(input);
    if let Err(e) = item {
        return e.to_compile_error();
    }
    let item = unsafe { item.unwrap_unchecked() };
    quote! { #item }
}
