use syn::{ExprCall, Type, parse_quote};

use crate::ext::Extension;

pub struct PluginGUI;
impl Extension for PluginGUI {
    fn vtable_type() -> Type {
        parse_quote! { ::clap_sys::ext::gui::clap_plugin_gui }
    }
    fn extension_pointer_constructor() -> ExprCall {
        parse_quote! {
                ::clap_prototype::ext::gui::extension_pointer()
        }
    }
}
