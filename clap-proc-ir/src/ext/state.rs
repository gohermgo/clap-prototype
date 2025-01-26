use syn::{ExprCall, Type, parse_quote};

use crate::ext::Extension;

pub struct PluginState;
impl Extension for PluginState {
    fn extension_pointer_constructor() -> ExprCall {
        parse_quote! {
            ::clap_prototype::ext::state::extension_pointer()
        }
    }
    fn vtable_type() -> Type {
        parse_quote! {
            ::clap_sys::ext::state::clap_plugin_state
        }
    }
}
