use crate::ext::{ExtensionPrototype, ProtoPtr};
use crate::plugin::HasExtension;

use clap_sys::ext::audio_ports::{clap_audio_port_info, clap_plugin_audio_ports};
use clap_sys::plugin::clap_plugin;

pub trait PluginAudioPortsPrototype<'host>:
    ExtensionPrototype<'host, Base = clap_plugin_audio_ports>
{
    fn count(&self, is_input: bool) -> u32;
    fn get(&self, index: u32, is_input: bool) -> Option<&clap_audio_port_info>;
}

fn get_ext<'host, 'ext, P, E>(ptr: *const clap_plugin) -> Option<&'ext E>
where
    P: HasExtension<'host, clap_plugin_audio_ports, ExtensionType = E> + 'ext,
    E: PluginAudioPortsPrototype<'host>,
    'host: 'ext,
{
    println!("GET EXT AUDIO PORTS");
    let plugin = unsafe { ptr.as_ref() }?;
    let parent = unsafe { (plugin.plugin_data as *const P).as_ref() }?;
    Some(parent.get_extension())
}
#[inline(never)]
unsafe extern "C" fn count<'host, P, E>(plugin_ptr: *const clap_plugin, is_input: bool) -> u32
where
    P: HasExtension<'host, clap_plugin_audio_ports, ExtensionType = E>,
    E: PluginAudioPortsPrototype<'host>,
{
    println!("EXTERN FN AUDIO PORTS COUNT");
    let Some(ext) = get_ext::<P, E>(plugin_ptr) else {
        return 0;
    };
    ext.count(is_input)
}

#[inline(never)]
unsafe extern "C" fn get<'host, P, E>(
    plugin: *const clap_plugin,
    index: u32,
    is_input: bool,
    info_dst: *mut clap_audio_port_info,
) -> bool
where
    P: HasExtension<'host, clap_plugin_audio_ports, ExtensionType = E>,
    E: PluginAudioPortsPrototype<'host>,
{
    println!("EXTERN FN AUDIO PORTS GET");
    let Some(ext) = get_ext::<P, E>(plugin) else {
        return false;
    };
    if let Some(info_src) = ext.get(index, is_input) {
        unsafe { core::ptr::copy_nonoverlapping(info_src, info_dst, 1) };
        true
    } else {
        false
    }
}
pub const fn vtable<'host, P, E>() -> &'static clap_plugin_audio_ports
where
    P: HasExtension<'host, clap_plugin_audio_ports, ExtensionType = E>,
    E: PluginAudioPortsPrototype<'host>,
{
    &clap_plugin_audio_ports {
        count: Some(count::<P, E>),
        get: Some(get::<P, E>),
    }
}
pub const fn extension_pointer<'host, P, E>() -> ProtoPtr<'host, E>
where
    P: HasExtension<'host, clap_plugin_audio_ports, ExtensionType = E>,
    E: PluginAudioPortsPrototype<'host, Parent = P>,
{
    let vt = vtable::<P, E>() as *const _;
    ProtoPtr(vt, ::core::marker::PhantomData)
}
