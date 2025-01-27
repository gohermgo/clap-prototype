use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Acquire;

use proc_macro2::TokenStream as TokenStream2;

use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::{LitFloat, parse_quote};

use syn::punctuated::Punctuated;
use syn::{Expr, Ident, LitCStr, LitInt, Token};

static PARSED: AtomicU32 = AtomicU32::new(0);
pub fn parse(input: TokenStream2) -> TokenStream2 {
    match syn::parse2(input) {
        Ok(PluginParamInfo(mut values)) => {
            let id = PARSED.fetch_add(1, Acquire).to_token_stream();
            println!("ID {id}");
            let id: LitInt = parse_quote! { #id };
            values.push(PluginParamInfoField::Id(id));
            quote! {
                ::clap_sys::ext::params::clap_param_info {
                    #values
                }
            }
        }
        Err(e) => e.to_compile_error(),
    }
}

// pub id: clap_id,
// pub flags: clap_param_info_flags,
// pub cookie: *mut c_void,
// pub name: [c_char; CLAP_NAME_SIZE],
// pub module: [c_char; CLAP_PATH_SIZE],
// pub min_value: f64,
// pub max_value: f64,
// pub default_value: f64,
pub struct PluginParamInfo(pub Punctuated<PluginParamInfoField, Token![,]>);
impl Parse for PluginParamInfo {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut fields = Punctuated::new();
        let flags = {
            let mut flags = Punctuated::new();
            if input.peek(Ident) && input.peek2(Token![,]) {
                let val: Ident = input.parse()?;
                flags.push(val);
            } else {
                while input.peek(Ident) && input.peek2(Token![|]) {
                    let val: Ident = input.parse()?;
                    let _: Token![|] = input.parse()?;
                    flags.push(val);
                }
                let last_flag: Ident = input.parse()?;
                flags.push(last_flag);
            }
            PluginParamInfoField::Flags(flags)
        };
        fields.push(flags);
        let _: Token![,] = input.parse()?;
        fields.push(PluginParamInfoField::Cookie(input.parse()?));
        let _: Token![,] = input.parse()?;
        fields.push(PluginParamInfoField::Name(input.parse()?));
        let _: Token![,] = input.parse()?;
        fields.push(PluginParamInfoField::Module(input.parse()?));
        let _: Token![,] = input.parse()?;
        fields.push(PluginParamInfoField::MinValue(input.parse()?));
        let _: Token![,] = input.parse()?;
        fields.push(PluginParamInfoField::MaxValue(input.parse()?));
        if input.peek(Token![,]) && input.peek2(LitFloat) {
            let _: Token![,] = input.parse()?;
            fields.push(PluginParamInfoField::DefaultValue(input.parse()?));
        } else {
            fields.push(PluginParamInfoField::DefaultValue(parse_quote! { 0.0 }));
        };
        Ok(PluginParamInfo(fields))
    }
}
pub enum PluginParamInfoField {
    Id(LitInt),
    Flags(Punctuated<Ident, Token![|]>),
    Cookie(Expr),
    Name(LitCStr),
    Module(LitCStr),
    MinValue(LitFloat),
    MaxValue(LitFloat),
    DefaultValue(LitFloat),
}
impl ToTokens for PluginParamInfoField {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        use PluginParamInfoField::*;
        let buf = match self {
            Id(id) => quote! { id: #id },
            Flags(flags) => quote! { flags: #flags },
            Cookie(c) => quote! { cookie: #c },
            Name(n) => {
                quote! { name: ::clap_prototype::plugin::PluginName::from_c_str(#n).to_fixed() }
            }
            Module(m) => {
                quote! { module: ::clap_prototype::plugin::PluginPath::from_c_str(#m).to_fixed() }
            }

            MaxValue(m) => quote! { max_value: #m },
            MinValue(m) => quote! { min_value: #m },
            DefaultValue(d) => quote! { default_value: #d },
        };
        tokens.extend(buf);
    }
}
