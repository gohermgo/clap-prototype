//! Implementation details of transparent types' Intermediate representation
//!

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};

use syn::{Token, parse_quote};

use syn::{AngleBracketedGenericArguments, Lifetime};
use syn::{Attribute, Expr, Ident, Type};

use crate::ir::implementation::IntoTraitImpl;

use super::implementation::DerefImpl;

pub struct TransparentDerefImpl<'wrapper, 'entry> {
    pub implementor: &'wrapper WrapperDefinition<'entry>,
    pub implementor_type: Type,
}
impl IntoTraitImpl for TransparentDerefImpl<'_, '_> {
    fn function_body(&self) -> Expr {
        parse_quote! {
            ::core::ops::Deref::deref(&self.0)
        }
    }
    fn implementor_type(&self) -> Type {
        self.implementor_type.clone()
    }
    fn lifetime(&self) -> Option<&Lifetime> {
        self.implementor.lifetime.as_ref()
    }
}
impl ToTokens for TransparentDerefImpl<'_, '_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let TransparentDerefImpl {
            implementor: WrapperDefinition { wrapped_type, .. },
            ..
        } = self;
        let target_type: Type = parse_quote! {
            <#wrapped_type as ::core::ops::Deref>::Target
        };
        let deref_impl = self.deref_impl_with(target_type);
        tokens.extend(quote! {#deref_impl});
    }
}
pub struct TransparentAsRefImpl<'wrapper, 'entry> {
    pub implementor: &'wrapper WrapperDefinition<'entry>,
    pub implementor_type: Type,
}
impl IntoTraitImpl for TransparentAsRefImpl<'_, '_> {
    fn function_body(&self) -> Expr {
        parse_quote! {
            ::core::convert::AsRef::<T>::as_ref(&self.0)
        }
    }
    fn implementor_type(&self) -> Type {
        self.implementor_type.clone()
    }
    fn lifetime(&self) -> Option<&Lifetime> {
        self.implementor.lifetime.as_ref()
    }
}
impl ToTokens for TransparentAsRefImpl<'_, '_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let TransparentAsRefImpl {
            implementor: WrapperDefinition { wrapped_type, .. },
            ..
        } = self;
        let target_type = parse_quote! { T };
        let impl_generic = Some(parse_quote! { <T> });
        let where_clause = parse_quote! {
            where
                #wrapped_type: ::core::convert::AsRef<T>
        };
        let as_ref_impl = self.as_ref_impl_with(target_type, impl_generic, where_clause);
        tokens.extend(quote! {#as_ref_impl});
    }
}
pub struct WrapperDefinition<'entry> {
    pub name: &'entry Ident,
    pub wrapped_type: Type,
    pub lifetime: Option<Lifetime>,
}
impl<'entry> WrapperDefinition<'entry> {
    pub fn unzip_lifetime(&self) -> (Option<AngleBracketedGenericArguments>, Option<Lifetime>) {
        let WrapperDefinition { lifetime, .. } = self;
        lifetime
            .as_ref()
            .map(|val| (parse_quote!(<#val>), parse_quote!(#val)))
            .unzip()
    }
    pub fn implementor_type(&self) -> Type {
        let WrapperDefinition { name, lifetime, .. } = self;
        let maybe_leading_reference: Option<Token![&]> =
            lifetime.is_none().then(|| parse_quote! { & });
        println!(
            "Leading reference is some {}",
            maybe_leading_reference.is_some()
        );
        parse_quote! { #maybe_leading_reference #name }
    }
    pub fn deref_impl(&self, target_type: Type, function_body: Expr) -> DerefImpl {
        let implementor_type = self.implementor_type();
        DerefImpl {
            lifetime_generic: None,
            implementor_type,
            function_body,
            target_type,
        }
    }
    pub fn transparent_deref_impl(&self) -> TransparentDerefImpl<'_, 'entry> {
        let implementor_type = self.implementor_type();
        TransparentDerefImpl {
            implementor: self,
            implementor_type,
        }
    }
    pub fn as_ref_impl(&self) -> TransparentAsRefImpl<'_, 'entry> {
        let name = self.name;
        TransparentAsRefImpl {
            implementor: self,
            implementor_type: parse_quote! { #name },
        }
    }
}
impl ToTokens for WrapperDefinition<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let WrapperDefinition {
            name, wrapped_type, ..
        } = self;

        let attr: Attribute = parse_quote! {
            #[repr(transparent)]
        };

        let (generics, lifetime) = self.unzip_lifetime();
        let tokenized = quote! {
            #attr
            pub struct #name #generics(#lifetime #wrapped_type);
        };
        tokens.extend(tokenized);
    }
}
