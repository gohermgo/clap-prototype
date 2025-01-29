use crate::ProtoPtr;
use crate::plugin::PluginPrototype;

use super::ExtensionPrototype;
pub use clap_proc_tools::plugin_parameter;
use clap_sys::{
    ext::state_context::{
        CLAP_STATE_CONTEXT_FOR_DUPLICATE, CLAP_STATE_CONTEXT_FOR_PRESET,
        CLAP_STATE_CONTEXT_FOR_PROJECT, clap_plugin_state_context, clap_plugin_state_context_type,
    },
    plugin::clap_plugin,
    stream::{clap_istream, clap_ostream},
};

#[repr(u32)]
pub enum PluginStateContextVariant {
    Preset = CLAP_STATE_CONTEXT_FOR_PRESET,
    Duplicate = CLAP_STATE_CONTEXT_FOR_DUPLICATE,
    Project = CLAP_STATE_CONTEXT_FOR_PROJECT,
}

pub trait PluginStateContextPrototype<'host>:
    ExtensionPrototype<'host, Base = clap_plugin_state_context>
{
    fn save(&self, output_stream: &clap_ostream, variant: PluginStateContextVariant) -> bool;
    fn load(&self, input_stream: &clap_istream, variant: PluginStateContextVariant) -> bool;
}
fn get_ext<'host, 'ext, P>(ptr: *const clap_plugin) -> Option<&'ext P>
where
    P: PluginStateContextPrototype<'host>,
    P::Parent: PluginPrototype<'host, PluginStateContextExtension = P>,
    'host: 'ext,
{
    println!("PARAMS ACCESS");
    let plugin = unsafe { ptr.as_ref() }?;
    let parent = unsafe { (plugin.plugin_data as *const P::Parent).as_ref() }?;
    parent.get_state_context_extension()
}
unsafe extern "C" fn save<'host, P>(
    plugin: *const clap_plugin,
    output_stream: *const clap_ostream,
    context_variant: clap_plugin_state_context_type,
) -> bool
where
    P: PluginStateContextPrototype<'host>,
    P::Parent: PluginPrototype<'host, PluginStateContextExtension = P>,
{
    let Some(p) = get_ext::<P>(plugin) else {
        return false;
    };
    let Some(output_stream) = (unsafe { output_stream.as_ref() }) else {
        return false;
    };
    p.save(output_stream, unsafe {
        ::core::mem::transmute(context_variant)
    })
}
unsafe extern "C" fn load<'host, P>(
    plugin: *const clap_plugin,
    input_stream: *const clap_istream,
    context_variant: clap_plugin_state_context_type,
) -> bool
where
    P: PluginStateContextPrototype<'host>,
    P::Parent: PluginPrototype<'host, PluginStateContextExtension = P>,
{
    let Some(p) = get_ext::<P>(plugin) else {
        return false;
    };
    let Some(input_stream) = (unsafe { input_stream.as_ref() }) else {
        return false;
    };
    p.load(input_stream, unsafe {
        ::core::mem::transmute(context_variant)
    })
}

pub const fn vtable<'host, P>() -> &'static clap_plugin_state_context
where
    P: PluginStateContextPrototype<'host>,
    P::Parent: PluginPrototype<'host, PluginStateContextExtension = P>,
{
    &clap_plugin_state_context {
        save: Some(save::<'host, P>),
        load: Some(load::<'host, P>),
    }
}
pub const fn extension_pointer<'host, P>() -> ProtoPtr<'host, P>
where
    P: PluginStateContextPrototype<'host>,
    P::Parent: PluginPrototype<'host, PluginStateContextExtension = P>,
{
    let vt = vtable::<P>() as *const _;
    ProtoPtr(vt, ::core::marker::PhantomData)
}
