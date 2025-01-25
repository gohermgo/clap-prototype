//! Intermediate representation
//!
//!

pub mod implementation;
pub mod transparent;

use implementation::{AsRefImpl, DerefImpl};
use proc_macro2::TokenStream as TokenStream2;

use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Token, parse_quote};

use syn::{AngleBracketedGenericArguments, Ident};

pub struct EzCStr {
    pub name: Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
}
impl Parse for EzCStr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        println!("Parsed new EzCStr {name:?}");
        let mut generics = None;
        if input.peek(Token![<]) {
            let generic_arguments = input.parse()?;
            generics = Some(generic_arguments);
        };
        Ok(EzCStr { name, generics })
    }
}
impl EzCStr {
    #[inline(always)]
    fn gen_inner(&self, r#struct: transparent::WrapperDefinition<'_>) -> TokenStream2 {
        println!("Generating EzCStr");
        let EzCStr { name, .. } = self;
        let as_ref_impl = AsRefImpl {
            lifetime_generic: None,
            implementor_type: parse_quote! {#name},
            function_body: parse_quote! {
                ::core::convert::AsRef::<T>::as_ref(&self.0)
            },
            target_type: parse_quote! {T},
            impl_generic: Some(parse_quote! {<T>}),
            where_clause: parse_quote! {
                where
                    ::core::ffi::CStr: AsRef<T>
            },
        };
        let deref_impl = DerefImpl {
            lifetime_generic: None,
            implementor_type: parse_quote! {#name},
            function_body: parse_quote! { &self.0 },
            target_type: parse_quote! { ::core::ffi::CStr },
        };
        quote! {
            #r#struct
            #as_ref_impl
            #deref_impl
        }
    }
    #[inline(always)]
    fn reference_wrapper(&self) -> transparent::WrapperDefinition<'_> {
        println!("Generating reference wrapper for EzCStr");
        transparent::WrapperDefinition {
            lifetime: Some(parse_quote!('a)),
            name: &self.name,
            wrapped_type: parse_quote! {::core::ffi::CStr },
        }
    }
    pub fn gen_reference_wrapper(&self) -> TokenStream2 {
        self.gen_inner(self.reference_wrapper())
    }
    #[inline(always)]
    fn unsized_wrapper(&self) -> transparent::WrapperDefinition<'_> {
        println!("Generating unsized wrapper for EzCStr");
        transparent::WrapperDefinition {
            name: &self.name,
            wrapped_type: parse_quote! { ::core::ffi::CStr },
            lifetime: None,
        }
    }
    pub fn gen_unsized_wrapper(&self) -> TokenStream2 {
        let wrapper = self.unsized_wrapper();
        let name = wrapper.name;
        let wrap_conversions = quote! {
            impl From<&#name> for ::std::rc::Rc<#name> {

                #[inline(always)]
                fn from(value: &#name) -> ::std::rc::Rc<#name> {
                    let value: ::std::rc::Rc<::core::ffi::CStr> = value.as_ref().into();
                    unsafe { ::core::mem::transmute(value) }
                }
            }
            impl From<&#name> for ::std::sync::Arc<#name> {

                #[inline(always)]
                fn from(value: &#name) -> ::std::sync::Arc<#name> {
                    let inner: ::std::sync::Arc<::core::ffi::CStr> = value.as_ref().into();
                    unsafe { ::core::mem::transmute(inner) }
                }
            }
            impl #name {
                #[inline(always)]
                pub const fn from_c_str(s: &::core::ffi::CStr) -> &#name {
                    unsafe { ::core::mem::transmute(s) }
                }
            }
        };
        let mut buf = self.gen_inner(wrapper);
        buf.extend(wrap_conversions);
        buf
    }
}
impl ToTokens for EzCStr {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(if self.generics.is_some() {
            self.gen_reference_wrapper()
        } else {
            self.gen_unsized_wrapper()
        });
    }
}
