use crate::{
    plugin::{PluginName, PluginPrototype},
    AbstractPrototype,
};
use clap_sys::{
    factory::plugin_factory::clap_plugin_factory,
    host::clap_host,
    plugin::{clap_plugin, clap_plugin_descriptor},
};
use core::{ffi::CStr, ptr::null};
use std::{ffi::c_char, sync::Arc};
pub trait PluginFactoryPrototype<'host> {
    type Produced: PluginPrototype<'host, Base = clap_plugin>;
    fn get_plugin_count(&self) -> u32;
    fn get_plugin_descriptor(&self, plugin_idx: u32) -> Option<&'static clap_plugin_descriptor>;
    fn create_plugin<'id>(
        &self,
        host: &'host clap_host,
        plugin_id: &'id PluginName,
    ) -> Option<Self::Produced>;
}

unsafe extern "C" fn get_plugin_count<'host, F>(factory_ptr: *const clap_plugin_factory) -> u32
where
    F: PluginFactoryPrototype<'host>,
{
    let Some(factory) = (unsafe { (factory_ptr as *const F).as_ref() }) else {
        println!("NULL FACTORY");
        return 0;
    };
    factory.get_plugin_count()
}
unsafe extern "C" fn get_plugin_descriptor<'host, F>(
    factory_ptr: *const clap_plugin_factory,
    plugin_idx: u32,
) -> *const clap_plugin_descriptor
where
    F: PluginFactoryPrototype<'host>,
{
    let Some(factory) = (unsafe { (factory_ptr as *const F).as_ref() }) else {
        println!("NULL FACTORY");
        return ::core::ptr::null();
    };
    let Some(descriptor) = factory.get_plugin_descriptor(plugin_idx) else {
        println!("NULL DESCRIPTOR");
        return ::core::ptr::null();
    };
    descriptor as *const _ as *const _
}
unsafe extern "C" fn create_plugin<'host, F>(
    factory_ptr: *const clap_plugin_factory,
    host_ptr: *const clap_host,
    plugin_id: *const c_char,
) -> *const clap_plugin
where
    F: PluginFactoryPrototype<'host>,
    F::Produced: PluginPrototype<'host, Base = clap_plugin>,
{
    println!("CREATE PLUGIN");
    let Some(factory) = (unsafe { (factory_ptr as *const F).as_ref() }) else {
        println!("NULL FACTORY");
        return ::core::ptr::null();
    };
    let Some(host) = (unsafe { host_ptr.as_ref() }) else {
        println!("NULL HOST");
        return ::core::ptr::null();
    };
    let plugin_id = match plugin_id.try_into() {
        Ok(val) => val,
        Err(e) => {
            println!("ERR {e:?}");
            return ::core::ptr::null();
        }
    };
    // Some really hairy shit coming up
    let Some(plugin_interface) = factory.create_plugin(host, plugin_id) else {
        println!("NO PLUGIN");
        return ::core::ptr::null();
    };
    // Leak the MF, let's hope
    let plugin = Box::leak(Box::new(plugin_interface));
    let addr = plugin as *mut _ as *mut ::core::ffi::c_void;
    let vt = unsafe { (plugin as *mut _ as *mut clap_plugin).as_mut() };
    if vt.is_none() {
        println!("VT SHIT");
        return ::core::ptr::null();
    }
    let vt = unsafe { vt.unwrap_unchecked() };
    vt.plugin_data = addr;
    vt
}
pub const fn vtable<'host, F>() -> &'static clap_plugin_factory
where
    F: PluginFactoryPrototype<'host>,
    F::Produced: PluginPrototype<'host, Base = clap_plugin>,
{
    &clap_plugin_factory {
        get_plugin_count: Some(get_plugin_count::<'host, F>),
        get_plugin_descriptor: Some(get_plugin_descriptor::<'host, F>),
        create_plugin: Some(create_plugin::<'host, F>),
    }
}
