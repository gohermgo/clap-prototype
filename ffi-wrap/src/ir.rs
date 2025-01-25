//! # Intermediate representation
//!
//!

use proc_macro2::TokenStream as TokenStream2;

use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::{parse_quote, Expr, Lifetime, WhereClause};

use syn::{AngleBracketedGenericArguments, Attribute, Ident, Type};

#[derive(Debug)]
pub struct EzCStr {
    pub name: Ident,
}
impl Parse for EzCStr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(|name| EzCStr { name })
    }
}
impl EzCStr {
    fn struct_definition(&self) -> TransparentWrapperDefinition<'_> {
        TransparentWrapperDefinition {
            lifetime: Some(parse_quote!('a)),
            name: &self.name,
            wrapped_type: parse_quote!(::core::ffi::CStr),
        }
    }
}
impl ToTokens for EzCStr {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let r#struct = self.struct_definition();
        let 
    }
}
pub struct EzCString {
    pub name: Ident,
}
#[inline(always)]
fn elide_lifetime_optional(lt: Option<&Lifetime>) -> Option<Lifetime> {
    if lt.is_some() {
        Some(parse_quote!(<'_>))
    } else {
        None
    }
}
struct TransparentDerefImpl<'wrapper, 'entry> {
    pub implementor: &'wrapper TransparentWrapperDefinition<'entry>,
}
impl ToTokens for TransparentDerefImpl<'_, '_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let TransparentDerefImpl {
            implementor:
                TransparentWrapperDefinition {
                    name,
                    lifetime,
                    wrapped_type,
                    ..
                },
        } = self;

        let target_type: Type = parse_quote! {
            <#wrapped_type as ::core::ops::Deref>::Target;
        };

        let lifetime = elide_lifetime_optional(lifetime.as_ref());

        let body: Expr = parse_quote! {
            ::core::ops::Deref::deref(&self.0)
        };

        tokens.extend(quote! {
            impl ::core::ops::Deref for #name #lifetime {
                type Target = #target_type;

                #[inline]
                fn deref(&self) -> &Self::Target {
                    #body
                }
            }
        });
    }
}
struct TransparentAsRefImpl<'wrapper, 'entry> {
    pub implementor: &'wrapper TransparentWrapperDefinition<'entry>,
}
impl ToTokens for TransparentAsRefImpl<'_, '_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let TransparentAsRefImpl {
            implementor:
                TransparentWrapperDefinition {
                    lifetime,
                    name,
                    wrapped_type,
                    ..
                },
        } = self;

        let where_clause: WhereClause = parse_quote!{
            where
                #wrapped_type: ::core::convert::AsRef<T>
        };

        let lifetime = elide_lifetime_optional(lifetime.as_ref());

        let body: Expr = parse_quote!{
            ::core::convert::AsRef::<T>::as_ref(&self.0)
        };

        tokens.extend(quote!{
            impl<T> ::core::convert::AsRef<T> for #name #lifetime
            #where_clause
            {
                #[inline]
                fn as_ref(&self) -> &T {
                    #body
                }
            }
        });
    }
}
struct TransparentWrapperDefinition<'entry> {
    pub name: &'entry Ident,
    pub wrapped_type: Type,
    pub lifetime: Option<Lifetime>,
}
impl<'entry> TransparentWrapperDefinition<'entry> {
    pub fn unzip_lifetime(&self) -> (Option<AngleBracketedGenericArguments>, Option<Lifetime>) {
        let TransparentWrapperDefinition { lifetime, .. } = self;
        lifetime
            .as_ref()
            .map(|val| (parse_quote!(<#val>), parse_quote!(#val)))
            .unzip()
    }
    pub fn deref_impl(&self) -> TransparentDerefImpl<'_, 'entry> {
        TransparentDerefImpl { implementor: self }
    }
    pub fn as_ref_impl(&self) -> TransparentAsRefImpl<'_, 'entry> {
        TransparentAsRefImpl { implementor: self }
    }
}
impl ToTokens for TransparentWrapperDefinition<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let TransparentWrapperDefinition {
            name,
            wrapped_type,
            ..
        } = self;

        let attr: Attribute = parse_quote!{
            #[repr(transparent)]
        };

        let (generics, lifetime) = self.unzip_lifetime();

        tokens.extend(quote! {
            #attr
            pub struct #name #generics(#lifetime #wrapped_type);
        });
    }
}
struct UnsizedWrapperDefinition<'entry> {
    pub name: &'entry Ident,
    pub wrapped_type: Type,
}

impl ToTokens for TransparentWrapperDefinition<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        
    }
}