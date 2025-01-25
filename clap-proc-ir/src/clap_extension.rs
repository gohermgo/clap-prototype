use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Attribute, Field, Fields, FieldsUnnamed};
use syn::{FieldsNamed, Ident, ItemStruct};
use syn::{Generics, Token, Visibility};
use syn::{parenthesized, parse_quote};

pub fn parse(attrs: TokenStream2, input: TokenStream2) -> TokenStream2 {
    match syn::parse2(attrs) {
        Ok(ExtensionAttrs { extension }) if extension == "PluginParams" => {
            throw_tokens::<PluginParams>(input)
        }
        Ok(ExtensionAttrs { extension }) => {
            syn::Error::new(extension.span(), "Unrecognized class").to_compile_error()
        }
        Err(e) => e.to_compile_error(),
    }
}
fn throw_tokens<T>(input: TokenStream2) -> TokenStream2
where
    T: ToTokens + Parse,
{
    let res = syn::parse2(input);
    if let Err(e) = res {
        e.to_compile_error()
    } else {
        let val: T = unsafe { res.unwrap_unchecked() };
        quote! { #val }
    }
}
pub struct ExtensionAttrs {
    pub extension: Ident,
}
impl Parse for ExtensionAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let extension = input.parse()?;
        Ok(ExtensionAttrs { extension })
    }
}
pub struct PluginParams {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub fields: Fields,
    pub base_field: Field,
    pub inner_fields: FieldsUnnamed,
    pub abstract_prototype: TokenStream2,
}
fn abstract_prototype_impl(ident: &Ident, base: Ident) -> TokenStream2 {
    let base_ident = format_ident!("{ident}Inner");
    quote! {
        impl ::clap_prototype::AbstractPrototype<'host> for #base_ident<'host> {
            type Base = #base;
            fn as_base(&self) -> &Self::Base {
                unsafe {self.0.as_ref_unchecked()}
            }
        }
        impl ::clap_protoype::ExtensionPrototype<'host> for #base_ident<'host> {
            fn from_raw_plugin<'ptr>(ptr: *const ::clap_sys::plugin::clap_plugin) -> Option<&'ptr Self> {
                Self(ptr, ::core::marker::PhantomData)
            }
        }
        impl ::core::ops::Deref for #ident<'host> {
            type Target = #base;
            fn deref(&self) -> &Self::Target {
                self.__inner.as_base()
            }
        }
        unsafe extern "C" fn count<'host>(plugin_ptr: *const clap_plugin) -> u32 {
            
        }
    }
}
impl Parse for PluginParams {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ItemStruct {
            attrs,
            vis,
            ident,
            generics,
            fields,
            ..
        } = input.parse()?;

        let abstract_prototype = abstract_prototype_impl(&ident, parse_quote! {
            ::clap_sys::ext::params::clap_plugin_params
        });
        Ok(PluginParams {
            attrs,
            vis,
            ident,
            generics,
            fields,
            base_field: parse_quote! {
                __inner: ::clap_sys::ext::params::clap_plugin_params,
            },
            inner_fields: parse_quote! {
                (*const ::clap_sys::ext::params::clap_plugin_params, ::core::marker::PhantomData<&'host ()>)
            },
            abstract_prototype,
        })
    }
}
impl ToTokens for PluginParams {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let PluginParams {
            attrs,
            vis,
            ident,
            generics,
            fields,
            base_field,
            inner_fields,
            abstract_prototype,
        } = self;
        let Fields::Named(FieldsNamed { named, .. }) = fields else {
            return;
        };
        let base_ident = format_ident!("{}Inner", ident);
        tokens.extend(quote! {
            #[repr(transparent)]
            struct #base_ident<'host>#inner_fields;
            #(#attrs)*
            #vis #ident<'host> {
                #base_field
                #named
            }
            #abstract_prototype
        })
    }
}
