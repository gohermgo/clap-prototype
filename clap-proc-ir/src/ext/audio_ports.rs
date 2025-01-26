use syn::{ExprCall, Type, parse_quote};

use crate::ext::Extension;

pub struct PluginAudioPorts;
impl Extension for PluginAudioPorts {
    fn vtable_type() -> Type {
        parse_quote! { ::clap_sys::ext::audio_ports::clap_plugin_audio_ports }
    }
    fn extension_pointer_constructor() -> ExprCall {
        parse_quote! {
            ::clap_prototype::ext::audio_ports::extension_pointer()
        }
    }
}
