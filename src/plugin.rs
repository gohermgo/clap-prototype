pub(crate) mod descriptor;
pub mod features;
pub(crate) mod str_types;

use clap_sys::process::{clap_process, clap_process_status};
pub use descriptor::PluginDescriptor;
pub use str_types::*;

use crate::AbstractPrototype;
use crate::ext::ExtensionPrototype;

use clap_sys::plugin::clap_plugin;

pub trait PluginPrototype<'host>: AbstractPrototype<'host, Base = clap_plugin> {
    /// `main-thread`
    ///
    /// Must be called after creating the plugin.
    /// If init returns false, the host must destroy the plugin instance.
    /// If init returns true, then the plugin is initialized and in the deactivated state.
    ///
    /// Unlike in [PluginFactoryPrototype::create_plugin](crate::factory::PluginFactoryPrototype::create_plugin),
    /// in init you have complete access to the host
    /// and host extensions, so clap related setup activities should be done here rather than in
    /// create_plugin.
    fn init(&mut self) -> bool;
    /// `main-thread` & `!active`
    ///
    /// Activate and deactivate the plugin.
    /// In this call the plugin may allocate memory and prepare everything needed for the process
    /// call. The process's sample rate will be constant and process's frame count will included in
    /// the [min, max] range, which is bounded by `[1,` [`i32::MAX`]`]`.
    /// In this call the plugin may call host-provided methods marked `being-activated`.
    /// Once activated the latency and port configuration must remain constant, until deactivation.
    /// Returns true on success.
    fn activate(&self, sample_rate: f64, min_frames_count: u32, max_frames_count: u32) -> bool;
    /// `main-thread` & `active`
    fn deactivate(&self);
    /// `audio-thread` & `active` & `!processing`
    ///
    /// Call start processing before processing.
    /// Returns true on success.
    fn start_processing(&self) -> bool;

    /// `audio-thread` & `active` & `processing`
    ///
    /// Call stop processing before sending the plugin to sleep.
    fn stop_processing(&self);
    /// `audio-thread` & `active`
    ///
    /// - Clears all buffers, performs a full reset of the processing state (filters, oscillators,
    ///   envelopes, lfo, ...) and kills all voices.
    /// - The parameter's value remain unchanged.
    /// - [clap_process::steady_time] may jump backward.
    ///
    fn reset(&mut self);
    /// `audio-thread` & `active` & `processing`
    ///
    /// process audio, events, ...
    ///
    /// All the pointers coming from [`clap_process`] and its nested attributes,
    /// are valid until [`PluginPrototype::process`] returns.
    fn process(&self, process_details: &clap_process) -> Option<clap_process_status>;
}

pub trait HasExtension<'host, B>: PluginPrototype<'host> {
    type ExtensionType: ExtensionPrototype<'host, Parent = Self, Base = B>;
    fn get_extension(&self) -> &Self::ExtensionType;
}
