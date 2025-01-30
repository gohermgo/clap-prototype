//! State-context extension
//!
//! brief extended state handling
//!
//! This extension lets the host save and load the plugin state with different semantics depending
//! on the context.
//!
//! Briefly, when loading a preset or duplicating a device, the plugin may want to partially load
//! the state and initialize certain things differently, like handling limited resources or fixed
//! connections to external hardware resources.
//!
//! Save and Load operations may have a different context.
//! All three operations should be equivalent:
//! 1. clap_plugin_state_context.load(clap_plugin_state.save(), CLAP_STATE_CONTEXT_FOR_PRESET)
//! 2. clap_plugin_state.load(clap_plugin_state_context.save(CLAP_STATE_CONTEXT_FOR_PRESET))
//! 3. clap_plugin_state_context.load(
//!        clap_plugin_state_context.save(CLAP_STATE_CONTEXT_FOR_PRESET),
//!        CLAP_STATE_CONTEXT_FOR_PRESET)
//!
//! If in doubt, fallback to clap_plugin_state.
//!
//! If the plugin implements CLAP_EXT_STATE_CONTEXT then it is mandatory to also implement
//! CLAP_EXT_STATE.
//!
//! It is unspecified which context is equivalent to clap_plugin_state.{save,load}()

pub use clap_proc_tools::plugin_parameter;

use core::mem::transmute;

use crate::ProtoPtr;

use crate::plugin::PluginPrototype;

use crate::ext::ExtensionPrototype;

use clap_sys::plugin::clap_plugin;

use clap_sys::ext::state_context::clap_plugin_state_context;
use clap_sys::ext::state_context::clap_plugin_state_context_type;
use clap_sys::ext::state_context::{
    CLAP_STATE_CONTEXT_FOR_DUPLICATE, CLAP_STATE_CONTEXT_FOR_PRESET, CLAP_STATE_CONTEXT_FOR_PROJECT,
};

use clap_sys::stream::{clap_istream, clap_ostream};

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
    let variant: PluginStateContextVariant = unsafe { transmute(context_variant) };
    p.save(output_stream, variant)
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
    let variant: PluginStateContextVariant = unsafe { transmute(context_variant) };
    p.load(input_stream, variant)
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
