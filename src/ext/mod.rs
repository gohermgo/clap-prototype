//pub mod gui;
use crate::Prototype;
use clap_sys::plugin::clap_plugin;
pub mod params;
pub trait ExtensionPrototype<'host>: Prototype<'host> {
    fn from_raw_plugin<'ptr>(plugin_ptr: *const clap_plugin) -> Option<&'ptr Self>;
}
