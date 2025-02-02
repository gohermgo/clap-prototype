//! State
//!
//! state management
//!
//! Plugins can implement this extension to save and restore both parameter
//! values and non-parameter state. This is used to persist a plugin's state
//! between project reloads, when duplicating and copying plugin instances, and
//! for host-side preset management.
//!
//! If you need to know if the save/load operation is meant for duplicating a plugin
//! instance, for saving/loading a plugin preset or while saving/loading the project
//! then consider implementing CLAP_EXT_STATE_CONTEXT in addition to CLAP_EXT_STATE

use crate::plugin::HasExtension;

use super::{ExtensionPrototype, ProtoPtr};
pub use clap_proc_tools::plugin_parameter;
use clap_sys::{
    ext::state::clap_plugin_state,
    plugin::clap_plugin,
    stream::{clap_istream, clap_ostream},
};
pub trait PluginStatePrototype<'host>: ExtensionPrototype<'host, Base = clap_plugin_state> {
    /// `main-thread`
    ///
    /// Saves the plugin state into stream.
    /// Returns true if the state was correctly saved.
    fn save(&self, output_stream: &clap_ostream) -> bool;
    /// `main-thread`
    ///
    /// Loads the plugin state from stream.
    /// Returns true if the state was correctly restored.
    fn load(&self, input_stream: &clap_istream) -> bool;
}
fn get_ext<'host, 'ext, P, E>(ptr: *const clap_plugin) -> Option<&'ext E>
where
    P: HasExtension<'host, clap_plugin_state, ExtensionType = E> + 'ext,
    E: PluginStatePrototype<'host, Parent = P, Base = clap_plugin_state>,
    'host: 'ext,
{
    let plugin = unsafe { ptr.as_ref() }?;
    let parent = unsafe { (plugin.plugin_data as *const E::Parent).as_ref() }?;
    Some(parent.get_extension())
}
unsafe extern "C" fn save<'host, P, E>(
    plugin: *const clap_plugin,
    output_stream: *const clap_ostream,
) -> bool
where
    P: HasExtension<'host, clap_plugin_state, ExtensionType = E>,
    E: PluginStatePrototype<'host, Parent = P, Base = clap_plugin_state>,
{
    let Some(p) = get_ext::<P, E>(plugin) else {
        return false;
    };
    let Some(output_stream) = (unsafe { output_stream.as_ref() }) else {
        return false;
    };
    p.save(output_stream)
}
unsafe extern "C" fn load<'host, P, E>(
    plugin: *const clap_plugin,
    input_stream: *const clap_istream,
) -> bool
where
    P: HasExtension<'host, clap_plugin_state, ExtensionType = E>,
    E: PluginStatePrototype<'host, Parent = P, Base = clap_plugin_state>,
{
    let Some(p) = get_ext::<P, E>(plugin) else {
        return false;
    };
    let Some(input_stream) = (unsafe { input_stream.as_ref() }) else {
        return false;
    };
    p.load(input_stream)
}

pub const fn vtable<'host, P, E>() -> &'static clap_plugin_state
where
    P: HasExtension<'host, clap_plugin_state, ExtensionType = E>,
    E: PluginStatePrototype<'host, Parent = P, Base = clap_plugin_state>,
{
    &clap_plugin_state {
        save: Some(save::<'host, P, E>),
        load: Some(load::<'host, P, E>),
    }
}
pub const fn extension_pointer<'host, P, E>() -> ProtoPtr<'host, E>
where
    P: HasExtension<'host, clap_plugin_state, ExtensionType = E>,
    E: PluginStatePrototype<'host, Parent = P, Base = clap_plugin_state>,
{
    let vt = vtable::<P, E>() as *const _;
    ProtoPtr(vt, ::core::marker::PhantomData)
}
