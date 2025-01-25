use crate::plugin::PluginPrototype;
//pub mod gui;
use crate::AbstractPrototype;
use clap_sys::plugin::{clap_plugin, clap_plugin_descriptor};
use std::pin;
use std::ptr::NonNull;
use std::sync::Arc;
pub mod params;
#[repr(transparent)]
pub struct ExtensionPointer<'host, E: AbstractPrototype<'host> + ?Sized>(
    *const E::Base,
    ::core::marker::PhantomData<&'host ()>,
);
impl<'host, B, E: ExtensionPrototype<'host, Base = B>> From<*const B>
    for ExtensionPointer<'host, E>
{
    #[inline]
    fn from(value: *const B) -> Self {
        ExtensionPointer(value, ::core::marker::PhantomData)
    }
}
impl<'host, E> ::core::ops::Deref for ExtensionPointer<'host, E>
where
    E: ExtensionPrototype<'host>,
{
    type Target = E;
    fn deref(&self) -> &Self::Target {
        unsafe {
            (self.0 as *const E)
                .as_ref()
                .expect("ExtensionPointer deref")
        }
    }
}
pub trait ExtensionPrototype<'host>: AbstractPrototype<'host> {
    type Parent: AbstractPrototype<'host>;
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
