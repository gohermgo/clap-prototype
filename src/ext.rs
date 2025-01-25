//pub mod gui;
use crate::AbstractPrototype;
use clap_sys::plugin::{clap_plugin, clap_plugin_descriptor};
use std::pin;
use std::ptr::NonNull;
use std::sync::Arc;
pub mod params;
pub trait ExtensionPrototype<'host>: AbstractPrototype<'host> {
    fn from_raw_plugin<'ptr>(plugin_ptr: *const clap_plugin) -> Option<&'ptr Self>;
}
fn create_extension_prototype<'host, E>(
    plugin_ptr: *const clap_plugin,
    extension_descriptor: *const clap_plugin_descriptor,
) -> Option<NonNull<clap_plugin>>
where
    E: ExtensionPrototype<'host>,
{
    let plugin = unsafe { plugin_ptr.as_ref() }?;
    let extension_descriptor = unsafe { extension_descriptor.as_ref() }?;
    let get_extension_fn = plugin.get_extension?;
    let extension_id = extension_descriptor.id;
    let returned_extension = unsafe { get_extension_fn(plugin_ptr, extension_id) };
    NonNull::new(returned_extension.cast_mut().cast())
}
