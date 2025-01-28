use core::cmp::Ordering;

use proc_macro2::TokenStream as TokenStream2;

use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, Token};

pub fn parse(input: TokenStream2) -> TokenStream2 {
    match syn::parse2(input) {
        Ok(ep @ PluginEntrypoint { .. }) => {
            quote! {#ep}
        }
        Err(e) => e.to_compile_error(),
    }
}
pub enum EntryFnIdent {
    Init,
    Deinit,
    GetFactory,
}
impl Parse for EntryFnIdent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if !input.peek(Ident) {
            return Err(syn::Error::new(
                input.span(),
                "Not an Ident, expected one of \"init\", \"deinit\", or \"get_factory\"",
            ));
        };

        let value: Ident = input.parse()?;

        if value == "init" {
            Ok(EntryFnIdent::Init)
        } else if value == "deinit" {
            Ok(EntryFnIdent::Deinit)
        } else if value == "get_factory" {
            Ok(EntryFnIdent::GetFactory)
        } else {
            Err(syn::Error::new_spanned(
                value,
                "Not an Ident, expected one of \"init\", \"deinit\", or \"get_factory\"",
            ))
        }
    }
}
pub struct EntryFn {
    pub ident: EntryFnIdent,
    pub _colon_token: Token![:],
    pub callee_ident: Ident,
}
impl Parse for EntryFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let _colon_token = input.parse()?;
        let callee_ident = input.parse()?;
        Ok(EntryFn {
            ident,
            _colon_token,
            callee_ident,
        })
    }
}
impl ToTokens for EntryFn {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let EntryFn {
            ident,
            _colon_token,
            callee_ident,
        } = self;
        let ffi_route = match ident {
            EntryFnIdent::Init => {
                quote! {

                unsafe extern "C" fn __plugin_entry_init(plugin_path: *const ::core::ffi::c_char) -> bool {
                    let plugin_path = <&::clap_prototype::plugin::PluginPath as ::core::convert::TryFrom<*const ::core::ffi::c_char>>::try_from(plugin_path);
                    match plugin_path {
                        Ok(path) => #callee_ident(path),
                        Err(e) => {
                            println!("plugin-entry-init: {e:?}");
                            false
                        }
                    }
                }
                    }
            }
            EntryFnIdent::Deinit => {
                quote! {
                    unsafe extern "C" fn __plugin_entry_deinit() {
                        #callee_ident()
                    }
                }
            }
            EntryFnIdent::GetFactory => {
                quote! {
                unsafe extern "C" fn __plugin_entry_get_factory(factory_id: *const ::core::ffi::c_char) -> *const ::core::ffi::c_void {
                    const fn __factory_assert<'host, T: ::clap_prototype::factory::PluginFactoryPrototype<'host>>(_: &'static T) {}
                    let factory_id = <&::clap_prototype::plugin::PluginName as ::core::convert::TryFrom<*const ::core::ffi::c_char>>::try_from(factory_id);
                    match factory_id {
                        Ok(id) => {
                            let factory: Option<&'static _> = #callee_ident(id);
                            if let Some(factory) = factory {
                                let _: () = __factory_assert(factory);
                                factory as *const _ as *const _
                            } else {
                                ::core::ptr::null()
                            }
                        },
                        Err(e) => {
                            println!("plugin-get-factory: {e:?}");
                            ::core::ptr::null()
                        }
                    }
                }
                }
            }
        };
        tokens.extend(quote! { #ffi_route });
    }
}
pub struct PluginEntrypoint(pub Punctuated<EntryFn, Token![,]>);

impl Parse for PluginEntrypoint {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let inner = Punctuated::parse_terminated(input)?;
        match PartialOrd::partial_cmp(&inner.len(), &3) {
            Some(Ordering::Equal) => Ok(PluginEntrypoint(inner)),
            Some(Ordering::Less) => Err(syn::Error::new_spanned(
                inner,
                "Missing an entrypoint function",
            )),
            Some(Ordering::Greater) => Err(syn::Error::new_spanned(
                inner,
                "Entrypoint has too many functions, there should \"init\", \"deinit\", and \"get_factory\"",
            )),
            None => panic!(),
        }
    }
}

impl ToTokens for PluginEntrypoint {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let PluginEntrypoint(list) = self;
        for entry in list {
            tokens.extend(quote! { #entry });
        }
        tokens.extend(quote!{
            #[allow(non_upper_case_globals)]
            #[unsafe(no_mangle)]
            pub static clap_entry: ::clap_sys::entry::clap_plugin_entry = ::clap_sys::entry::clap_plugin_entry {
                clap_version: ::clap_sys::version::CLAP_VERSION,
                init: Some(__plugin_entry_init),
                deinit: Some(__plugin_entry_deinit),
                get_factory: Some(__plugin_entry_get_factory)
            };
        });
    }
}
