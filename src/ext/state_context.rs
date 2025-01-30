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
use crate::plugin::HasExtension;

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
    /// suitable for storing and loading a state as a preset
    Preset = CLAP_STATE_CONTEXT_FOR_PRESET,

    /// suitable for duplicating a plugin instance
    Duplicate = CLAP_STATE_CONTEXT_FOR_DUPLICATE,
    /// suitable for storing and loading a state within a project/song
    Project = CLAP_STATE_CONTEXT_FOR_PROJECT,
}
pub trait PluginStateContextPrototype<'host>:
    ExtensionPrototype<'host, Base = clap_plugin_state_context>
{
    /// `main-thread`
    ///
    /// Saves the plugin state into stream, according to context_type.
    /// Returns true if the state was correctly saved.
    ///
    /// Note that the result may be loaded by both clap_plugin_state.load() and
    /// clap_plugin_state_context.load().
    fn save(&self, output_stream: &clap_ostream, variant: PluginStateContextVariant) -> bool;
    /// `main-thread`
    /// Loads the plugin state from stream, according to context_type.
    /// Returns true if the state was correctly restored.
    ///
    /// Note that the state may have been saved by clap_plugin_state.save() or
    /// clap_plugin_state_context.save() with a different context_type.
    fn load(&self, input_stream: &clap_istream, variant: PluginStateContextVariant) -> bool;
}
fn get_ext<'host, 'ext, P, E>(ptr: *const clap_plugin) -> Option<&'ext E>
where
    P: HasExtension<'host, clap_plugin_state_context, ExtensionType = E> + 'ext,
    E: PluginStateContextPrototype<'host>,
    'host: 'ext,
{
    println!("PARAMS ACCESS");
    let plugin = unsafe { ptr.as_ref() }?;
    let parent = unsafe { (plugin.plugin_data as *const P).as_ref() }?;
    Some(parent.get_extension())
}
unsafe extern "C" fn save<'host, P, E>(
    plugin: *const clap_plugin,
    output_stream: *const clap_ostream,
    context_variant: clap_plugin_state_context_type,
) -> bool
where
    P: HasExtension<'host, clap_plugin_state_context, ExtensionType = E>,
    E: PluginStateContextPrototype<'host>,
{
    let Some(p) = get_ext::<P, E>(plugin) else {
        return false;
    };
    let Some(output_stream) = (unsafe { output_stream.as_ref() }) else {
        return false;
    };
    let variant: PluginStateContextVariant = unsafe { transmute(context_variant) };
    p.save(output_stream, variant)
}
unsafe extern "C" fn load<'host, P, E>(
    plugin: *const clap_plugin,
    input_stream: *const clap_istream,
    context_variant: clap_plugin_state_context_type,
) -> bool
where
    P: HasExtension<'host, clap_plugin_state_context, ExtensionType = E>,
    E: PluginStateContextPrototype<'host>,
{
    let Some(p) = get_ext::<P, E>(plugin) else {
        return false;
    };
    let Some(input_stream) = (unsafe { input_stream.as_ref() }) else {
        return false;
    };
    let variant: PluginStateContextVariant = unsafe { transmute(context_variant) };
    p.load(input_stream, variant)
}

pub const fn vtable<'host, P, E>() -> &'static clap_plugin_state_context
where
    P: HasExtension<'host, clap_plugin_state_context, ExtensionType = E>,
    E: PluginStateContextPrototype<'host>,
{
    &clap_plugin_state_context {
        save: Some(save::<'host, P, E>),
        load: Some(load::<'host, P, E>),
    }
}
pub const fn extension_pointer<'host, P, E>() -> ProtoPtr<'host, E>
where
    P: HasExtension<'host, clap_plugin_state_context, ExtensionType = E>,
    E: PluginStateContextPrototype<'host>,
{
    let vt = vtable::<P, E>() as *const _;
    ProtoPtr(vt, ::core::marker::PhantomData)
}
