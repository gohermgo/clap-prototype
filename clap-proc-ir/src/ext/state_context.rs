use syn::{ExprCall, Type, parse_quote};

use crate::ext::Extension;

pub struct PluginStateContext;
impl Extension for PluginStateContext {
    fn extension_pointer_constructor() -> ExprCall {
        parse_quote! {
            ::clap_prototype::ext::state_context::extension_pointer()
        }
    }
    fn vtable_type() -> Type {
        parse_quote! {
            ::clap_sys::ext::state_context::clap_plugin_state_context
        }
    }
}
