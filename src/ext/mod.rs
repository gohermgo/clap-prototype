//pub mod gui;
use crate::AbstractPrototype;
use clap_sys::plugin::clap_plugin;
use std::pin;
pub mod params;
pub trait ExtensionPrototype<'host>: AbstractPrototype<'host> {
    fn from_raw_plugin<'ptr>(plugin_ptr: *const clap_plugin) -> Option<&'ptr Self>;
}
fn create_extension_prototype<'host, E>(plugin_ptr: *const clap_plugin) -> Option<Box<pin::Pin<E>>>
where
    E: ExtensionPrototype<'host>,
{
    let plugin = unsafe { plugin_ptr.as_ref() }?;
    let get_extension_fn = plugin.get_extension?;
    todo!()
}
