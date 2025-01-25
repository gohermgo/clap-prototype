//! Generics for implementations of traits for example
//!
//!

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{AngleBracketedGenericArguments, Expr, Lifetime, Type, WhereClause, parse_quote};

#[inline(always)]
fn elide_lifetime_optional(lt: Option<&Lifetime>) -> Option<AngleBracketedGenericArguments> {
    if lt.is_some() {
        Some(parse_quote! {<'_>})
    } else {
        None
    }
}

pub struct TraitImpl {
    pub lifetime_generic: Option<AngleBracketedGenericArguments>,
    pub implementor_type: Type,
    pub function_body: Expr,
}
pub trait IntoTraitImpl {
    fn implementor_type(&self) -> Type;
    fn lifetime(&self) -> Option<&Lifetime>;
    fn function_body(&self) -> Expr;
    fn trait_impl(&self) -> TraitImpl {
        let lifetime_generic = elide_lifetime_optional(self.lifetime());
        TraitImpl {
            function_body: self.function_body(),
            implementor_type: self.implementor_type(),
            lifetime_generic,
        }
    }
    fn deref_impl_with(&self, target_type: Type) -> DerefImpl {
        let TraitImpl {
            lifetime_generic,
            implementor_type,
            function_body,
        } = self.trait_impl();
        DerefImpl {
            lifetime_generic,
            implementor_type,
            function_body,
            target_type,
        }
    }
    fn as_ref_impl_with(
        &self,
        target_type: Type,
        impl_generic: Option<AngleBracketedGenericArguments>,
        where_clause: Option<WhereClause>,
    ) -> AsRefImpl {
        let TraitImpl {
            lifetime_generic,
            implementor_type,
            function_body,
        } = self.trait_impl();
        AsRefImpl {
            lifetime_generic,
            implementor_type,
            function_body,
            target_type,
            impl_generic,
            where_clause,
        }
    }
}
pub struct DerefImpl {
    pub lifetime_generic: Option<AngleBracketedGenericArguments>,
    pub implementor_type: Type,
    pub function_body: Expr,
    pub target_type: Type,
}
impl ToTokens for DerefImpl {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let DerefImpl {
            lifetime_generic,
            implementor_type,
            function_body,
            target_type,
        } = self;
        tokens.extend(quote! {
            impl ::core::ops::Deref for #implementor_type #lifetime_generic {
                type Target = #target_type;

                #[inline]
                fn deref(&self) -> &Self::Target {
                    #function_body
                }
            }
        });
    }
}
pub struct AsRefImpl {
    pub lifetime_generic: Option<AngleBracketedGenericArguments>,
    pub implementor_type: Type,
    pub function_body: Expr,
    pub target_type: Type,
    pub impl_generic: Option<AngleBracketedGenericArguments>,
    pub where_clause: Option<WhereClause>,
}
impl ToTokens for AsRefImpl {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let AsRefImpl {
            lifetime_generic,
            implementor_type,
            function_body,
            target_type,
            impl_generic,
            where_clause,
        } = self;
        tokens.extend(quote! {
            impl #impl_generic ::core::convert::AsRef<#target_type> for #implementor_type #lifetime_generic
            #where_clause
            {
                #[inline]
                fn as_ref(&self) -> &#target_type {
                    #function_body
                }
            }
        });
    }
}
