use crate::ext::{ExtensionPrototype, ProtoPtr};
use crate::plugin::PluginPrototype;

use clap_sys::ext::audio_ports::{clap_audio_port_info, clap_plugin_audio_ports};
use clap_sys::plugin::clap_plugin;

pub trait PluginAudioPortsPrototype<'host>:
    ExtensionPrototype<'host, Base = clap_plugin_audio_ports>
{
    fn count(&self, is_input: bool) -> u32;
    fn get(&self, index: u32, is_input: bool) -> Option<&clap_audio_port_info>;
}

fn get_ext<'host, 'ext, P>(ptr: *const clap_plugin) -> Option<&'ext P>
where
    P: PluginAudioPortsPrototype<'host>,
    P::Parent: PluginPrototype<'host, PluginAudioPortsExtension = P>,
    'host: 'ext,
{
    println!("GET EXT AUDIO PORTS");
    let plugin = unsafe { ptr.as_ref() }?;
    let parent = unsafe { (plugin.plugin_data as *const P::Parent).as_ref() }?;
    parent.get_audio_ports_extension()
}
#[inline(never)]
unsafe extern "C" fn count<'host, P>(plugin_ptr: *const clap_plugin, is_input: bool) -> u32
where
    P: PluginAudioPortsPrototype<'host>,
    P::Parent: PluginPrototype<'host, PluginAudioPortsExtension = P>,
{
    println!("EXTERN FN AUDIO PORTS COUNT");
    let Some(ext) = get_ext::<P>(plugin_ptr) else {
        return 0;
    };
    ext.count(is_input)
}

#[inline(never)]
unsafe extern "C" fn get<'host, P>(
    plugin: *const clap_plugin,
    index: u32,
    is_input: bool,
    info_dst: *mut clap_audio_port_info,
) -> bool
where
    P: PluginAudioPortsPrototype<'host>,
    P::Parent: PluginPrototype<'host, PluginAudioPortsExtension = P>,
{
    println!("EXTERN FN AUDIO PORTS GET");
    let Some(ext) = get_ext::<P>(plugin) else {
        return false;
    };
    if let Some(info_src) = ext.get(index, is_input) {
        unsafe { core::ptr::copy_nonoverlapping(info_src, info_dst, 1) };
        true
    } else {
        false
    }
}
pub const fn vtable<'host, P>() -> &'static clap_plugin_audio_ports
where
    P: PluginAudioPortsPrototype<'host>,
    P::Parent: PluginPrototype<'host, PluginAudioPortsExtension = P>,
{
    &clap_plugin_audio_ports {
        count: Some(count::<P>),
        get: Some(get::<P>),
    }
}
pub const fn extension_pointer<'host, P>() -> ProtoPtr<'host, P>
where
    P: PluginAudioPortsPrototype<'host>,
    P::Parent: PluginPrototype<'host, PluginAudioPortsExtension = P>,
{
    let vt = vtable::<P>() as *const _;
    ProtoPtr(vt, ::core::marker::PhantomData)
}
