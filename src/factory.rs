use crate::{plugin::PluginPrototype, AbstractPrototype};
use clap_sys::{
    factory::plugin_factory::clap_plugin_factory,
    host::clap_host,
    plugin::{clap_plugin, clap_plugin_descriptor},
};
use core::{ffi::CStr, ptr::null};
use std::ffi::c_char;
pub trait PluginFactoryPrototype<'host>:
    AbstractPrototype<'host, Base = clap_plugin_factory>
{
    /// Associated type referring to the
    /// type of plugin this factory produces
    type Produced: PluginPrototype<'host> + 'host;
    fn get_plugin_count(&self) -> u32;
    fn get_plugin_descriptor(&self, plugin_idx: u32) -> Option<&'static clap_plugin_descriptor>;
    fn create_plugin<'id>(
        &self,
        host: &'host clap_host,
        plugin_id: &'id CStr,
    ) -> Option<&'host Self::Produced>
    where
        'host: 'id;
}
unsafe extern "C" fn get_plugin_count<'host, F>(factory_ptr: *const clap_plugin_factory) -> u32
where
    F: PluginFactoryPrototype<'host>,
{
    (factory_ptr as *const F)
        .as_ref()
        .map(|factory| factory.get_plugin_count())
        .unwrap_or_default()
}
unsafe extern "C" fn get_plugin_descriptor<'host, F>(
    factory_ptr: *const clap_plugin_factory,
    plugin_idx: u32,
) -> *const clap_plugin_descriptor
where
    F: PluginFactoryPrototype<'host>,
{
    (factory_ptr as *const F)
        .as_ref()
        .and_then(|factory| factory.get_plugin_descriptor(plugin_idx))
        .map(|descriptor| descriptor as *const _)
        .unwrap_or(null())
}
unsafe extern "C" fn create_plugin<'host, F>(
    factory_ptr: *const clap_plugin_factory,
    host_ptr: *const clap_host,
    plugin_id: *const c_char,
) -> *const clap_plugin
where
    F: PluginFactoryPrototype<'host>,
{
    (factory_ptr as *const F)
        .as_ref()
        .and_then(|factory| {
            factory.create_plugin(unsafe { host_ptr.as_ref() }?, unsafe {
                CStr::from_ptr(plugin_id)
            })
        })
        .map(|plugin| plugin.as_base() as *const _)
        .unwrap_or(null())
}
pub const fn vtable<'host, F>() -> &'static clap_plugin_factory
where
    F: PluginFactoryPrototype<'host>,
{
    &clap_plugin_factory {
        get_plugin_count: Some(get_plugin_count::<'host, F>),
        get_plugin_descriptor: Some(get_plugin_descriptor::<'host, F>),
        create_plugin: Some(create_plugin::<'host, F>),
    }
}
