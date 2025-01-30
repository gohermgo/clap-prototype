pub(crate) mod descriptor;
pub mod features;
pub(crate) mod str_types;

pub use descriptor::PluginDescriptor;
pub use str_types::*;

use crate::AbstractPrototype;
use crate::ext::ExtensionPrototype;

use clap_sys::events::clap_output_events;
use clap_sys::plugin::clap_plugin;

pub trait PluginPrototype<'host>: AbstractPrototype<'host, Base = clap_plugin> {
    fn initialize(&mut self) -> bool;
    fn get_descriptor(&self) -> PluginDescriptor<'_>;
    fn get_id(&self) -> &PluginID {
        self.get_descriptor().id
    }
    fn sync_main_thread_with_audio_thread(&self, output_events: &clap_output_events) -> bool;
    fn sync_proc_thread_with_main_thread(&self) -> bool;
}

pub trait HasExtension<'host, B>: PluginPrototype<'host> {
    type ExtensionType: ExtensionPrototype<'host, Parent = Self, Base = B>;
    fn get_extension(&self) -> &Self::ExtensionType;
}
