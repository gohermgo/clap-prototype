use crate::plugin::PluginPrototype;

use super::{ProtoPtr, ExtensionPrototype};
pub use clap_proc_tools::plugin_parameter;
use clap_sys::{
    ext::state::clap_plugin_state,
    plugin::clap_plugin,
    stream::{clap_istream, clap_ostream},
};
pub trait PluginStatePrototype<'host>: ExtensionPrototype<'host, Base = clap_plugin_state> {
    fn save(&self, output_stream: &clap_ostream) -> bool;
    fn load(&self, input_stream: &clap_istream) -> bool;
}
fn get_ext<'host, 'ext, P>(ptr: *const clap_plugin) -> Option<&'ext P>
where
    P: PluginStatePrototype<'host>,
    P::Parent: PluginPrototype<'host, PluginStateExtension = P>,
    'host: 'ext,
{
    println!("PARAMS ACCESS");
    let plugin = unsafe { ptr.as_ref() }?;
    let parent = unsafe { (plugin.plugin_data as *const P::Parent).as_ref() }?;
    parent.get_state_extension()
}
unsafe extern "C" fn save<'host, P>(
    plugin: *const clap_plugin,
    output_stream: *const clap_ostream,
) -> bool
where
    P: PluginStatePrototype<'host>,
    P::Parent: PluginPrototype<'host, PluginStateExtension = P>,
{
    let Some(p) = get_ext::<P>(plugin) else {
        return false;
    };
    let Some(output_stream) = (unsafe { output_stream.as_ref() }) else {
        return false;
    };
    p.save(output_stream)
}
unsafe extern "C" fn load<'host, P>(
    plugin: *const clap_plugin,
    input_stream: *const clap_istream,
) -> bool
where
    P: PluginStatePrototype<'host>,
    P::Parent: PluginPrototype<'host, PluginStateExtension = P>,
{
    let Some(p) = get_ext::<P>(plugin) else {
        return false;
    };
    let Some(input_stream) = (unsafe { input_stream.as_ref() }) else {
        return false;
    };
    p.load(input_stream)
}

pub const fn vtable<'host, P>() -> &'static clap_plugin_state
where
    P: PluginStatePrototype<'host>,
    P::Parent: PluginPrototype<'host, PluginStateExtension = P>,
{
    &clap_plugin_state {
        save: Some(save::<'host, P>),
        load: Some(load::<'host, P>),
    }
}
pub const fn extension_pointer<'host, P>() -> ProtoPtr<'host, P>
where
    P: PluginStatePrototype<'host>,
    P::Parent: PluginPrototype<'host, PluginStateExtension = P>,
{
    let vt = vtable::<P>() as *const _;
    ProtoPtr(vt, ::core::marker::PhantomData)
}
