use crate::{AbstractPrototype, ProtoPtr};
use clap_sys::plugin::{clap_plugin, clap_plugin_descriptor};
use std::ptr::NonNull;

pub mod audio_ports;
pub mod params;
pub mod state;

pub use clap_proc_tools::extends;

impl<'host, B, E: ExtensionPrototype<'host, Base = B>> From<*const B> for ProtoPtr<'host, E> {
    #[inline]
    fn from(value: *const B) -> Self {
        println!("yowch");
        ProtoPtr(value, ::core::marker::PhantomData)
    }
}
impl<'host, B, E: ExtensionPrototype<'host, Base = B>> From<&E> for ProtoPtr<'host, E> {
    #[inline]
    fn from(value: &E) -> Self {
        let base = value.as_base();
        ProtoPtr::from(base as *const B)
    }
}
impl<'host, E> ::core::ops::Deref for ProtoPtr<'host, E>
where
    E: ExtensionPrototype<'host>,
{
    type Target = E::Base;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref().expect("ExtensionPointer deref") }
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
