use syn::{ExprCall, Type, parse_quote};

use crate::ext::Extension;

pub struct PluginParams;
impl Extension for PluginParams {
    fn vtable_type() -> Type {
        parse_quote! { ::clap_sys::ext::params::clap_plugin_params }
    }
    fn extension_pointer_constructor() -> ExprCall {
        parse_quote! {
                ::clap_prototype::ext::params::extension_pointer()
        }
    }
}
