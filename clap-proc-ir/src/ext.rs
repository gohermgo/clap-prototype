pub mod audio_ports;
pub mod params;
pub mod state;

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};

use syn::parse::{Parse, ParseStream};
use syn::{ExprCall, parse_quote};

use syn::{Attribute, Field, Fields, Type};
use syn::{FieldsNamed, Ident, ItemStruct};
use syn::{Generics, Visibility};

use audio_ports::PluginAudioPorts;
use params::PluginParams;
use state::PluginState;

pub fn parse(attrs: TokenStream2, input: TokenStream2) -> TokenStream2 {
    match syn::parse2(attrs) {
        Err(e) => e.to_compile_error(),
        Ok(ExtensionAttrs { extension }) if extension == "PluginParams" => {
            ExtensionTokenizer::<PluginParams>::throw_tokenize(input)
        }
        Ok(ExtensionAttrs { extension }) if extension == "PluginAudioPorts" => {
            ExtensionTokenizer::<PluginAudioPorts>::throw_tokenize(input)
        }
        Ok(ExtensionAttrs { extension }) if extension == "PluginState" => {
            ExtensionTokenizer::<PluginState>::throw_tokenize(input)
        }
        Ok(ExtensionAttrs { extension }) => {
            syn::Error::new(extension.span(), "Unrecognized class").to_compile_error()
        }
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
pub struct ExtensionStruct<E: Extension> {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub fields: Fields,
    pub base_field: Field,
    _ext: ::core::marker::PhantomData<E>,
}

pub trait Extension {
    fn vtable_type() -> Type;
    fn extension_pointer_constructor() -> ExprCall;
    fn abstract_prototype_impl(extension_ident: &Ident) -> TokenStream2 {
        let vtable_type = Self::vtable_type();
        quote! {
            impl<'host> ::clap_prototype::AbstractPrototype<'host> for #extension_ident<'host> {
                type Base = #vtable_type;
                fn as_base(&self) -> &Self::Base {
                    ::core::ops::Deref::deref(&self.base)
                }
            }
            impl<'host> ::core::ops::Deref for #extension_ident<'host> {
                type Target = #vtable_type;
                fn deref(&self) -> &Self::Target {
                    self.as_base()
                }
            }
        }
    }
}
pub struct ExtensionPointerType(pub Type);
impl ExtensionPointerType {
    pub fn new(extension_ident: &Ident) -> ExtensionPointerType {
        parse_quote! {
            ::clap_prototype::ext::ExtensionPointer<'host, #extension_ident<'host>>
        }
    }
}
impl Parse for ExtensionPointerType {
    fn parse(input: ParseStream) -> syn::Result<ExtensionPointerType> {
        let inner: Type = input.parse()?;
        Ok(ExtensionPointerType(inner))
    }
}
impl ToTokens for ExtensionPointerType {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ExtensionPointerType(inner) = self;
        tokens.extend(quote! {#inner});
    }
}
pub struct ExtensionPointerDefinition {
    pub extension_pointer_ident: Ident,
    pub extension_pointer_type: ExtensionPointerType,
    pub extension_pointer_constructor: ExprCall,
}
impl ExtensionPointerDefinition {
    pub fn new<E: Extension>(extension_ident: &Ident) -> ExtensionPointerDefinition {
        ExtensionPointerDefinition {
            extension_pointer_ident: format_ident!("{extension_ident}Extension"),
            extension_pointer_type: ExtensionPointerType::new(extension_ident),
            extension_pointer_constructor: E::extension_pointer_constructor(),
        }
    }
}
impl ToTokens for ExtensionPointerDefinition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            extension_pointer_ident,
            extension_pointer_type,
            extension_pointer_constructor,
        } = self;
        let definition: ItemStruct = parse_quote! {
            #[repr(transparent)]
            pub struct #extension_pointer_ident<'host>(#extension_pointer_type);
        };
        tokens.extend(quote! {#definition});
        let impl_body = quote! {
            impl<'host> #extension_pointer_ident<'host> {
                pub const fn new() -> Self {
                    Self(#extension_pointer_constructor)
                }
            }
            impl<'host> ::core::ops::Deref for #extension_pointer_ident<'host> {
                type Target = #extension_pointer_type;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        };
        tokens.extend(quote! {#impl_body})
    }
}
impl<E: Extension> Parse for ExtensionStruct<E> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ItemStruct {
            attrs,
            vis,
            ident,
            generics,
            fields,
            ..
        } = input.parse()?;
        let extension_pointer_type = format_ident!("{ident}Extension");
        let base_field = parse_quote! {
            base: #extension_pointer_type<'host>
        };
        Ok(ExtensionStruct {
            attrs,
            vis,
            ident,
            generics,
            fields,
            base_field,
            _ext: ::core::marker::PhantomData,
        })
    }
}
impl<E: Extension> ToTokens for ExtensionStruct<E> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ExtensionStruct {
            attrs,
            vis,
            ident,
            fields,
            base_field,
            ..
        } = self;
        let Fields::Named(FieldsNamed { named, .. }) = fields else {
            return;
        };
        tokens.extend(quote! {
            #(#attrs)*
            #[repr(C)]
            #vis struct #ident<'host> {
                #base_field,
                #named
            }
            unsafe impl<'host> ::core::marker::Send for #ident<'host> {}
            unsafe impl<'host> ::core::marker::Sync for #ident<'host> {}
        })
    }
}
pub struct ExtensionTokenizer<E: Extension>(pub ExtensionStruct<E>);
impl<E: Extension> ExtensionTokenizer<E> {
    pub fn throw_tokenize(input: TokenStream2) -> TokenStream2 {
        match syn::parse2(input) {
            Ok(ext @ ExtensionStruct::<E> { .. }) => {
                let tokenizer = ExtensionTokenizer(ext);
                quote! { #tokenizer }
            }
            Err(e) => e.to_compile_error(),
        }
    }
}
impl<E: Extension> ToTokens for ExtensionTokenizer<E> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ExtensionTokenizer(ext) = self;
        let abstract_prototype = E::abstract_prototype_impl(&ext.ident);
        let extension_pointer = ExtensionPointerDefinition::new::<E>(&ext.ident);
        tokens.extend(quote! {
            #abstract_prototype
            #extension_pointer
            #ext
        })
    }
}
