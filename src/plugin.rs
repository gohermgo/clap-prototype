pub(crate) mod descriptor;
pub mod features;
pub(crate) mod str_types;

use clap_sys::ext::audio_ports::clap_plugin_audio_ports;
use clap_sys::ext::gui::clap_plugin_gui;
use clap_sys::ext::params::clap_plugin_params;
use clap_sys::ext::state::clap_plugin_state;
use clap_sys::ext::state_context::clap_plugin_state_context;
pub use descriptor::PluginDescriptor;
pub use str_types::*;

use crate::AbstractPrototype;
use crate::ext::ExtensionPrototype;
use crate::ext::audio_ports::PluginAudioPortsPrototype;
use crate::ext::gui::PluginGUIPrototype;
use crate::ext::params::PluginParamsPrototype;
use crate::ext::state::PluginStatePrototype;
use crate::ext::state_context::PluginStateContextPrototype;

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
    fn get_params_extension<E>(&self) -> Option<&E>
    where
        E: PluginParamsPrototype<'host, Parent = Self, Base = clap_plugin_params>,
    {
        None
    }
    fn get_audio_ports_extension<E>(&self) -> Option<&E>
    where
        E: PluginAudioPortsPrototype<'host, Parent = Self, Base = clap_plugin_audio_ports>,
    {
        None
    }
    fn get_state_extension<E>(&self) -> Option<&E>
    where
        E: PluginStatePrototype<'host, Parent = Self, Base = clap_plugin_state>,
    {
        None
    }
    fn get_state_context_extension<E>(&self) -> Option<&E>
    where
        E: PluginStateContextPrototype<'host, Parent = Self, Base = clap_plugin_state_context>,
    {
        None
    }
    fn get_gui_extension<E>(&self) -> Option<&E>
    where
        E: PluginGUIPrototype<'host, Parent = Self, Base = clap_plugin_gui>,
    {
        None
    }
}

pub trait HasExtension<'host, B>: PluginPrototype<'host> {
    type ExtensionType: ExtensionPrototype<'host, Parent = Self, Base = B>;
    fn get_extension(&self) -> &Self::ExtensionType;
}
