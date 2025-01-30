use crate::plugin::HasExtension;
use crate::plugin::PluginParameterValueText;

use super::{ExtensionPrototype, ProtoPtr};
pub use clap_proc_tools::plugin_parameter;
use clap_sys::{
    events::{clap_input_events, clap_output_events},
    ext::params::{clap_param_info, clap_plugin_params},
    id::clap_id,
    plugin::clap_plugin,
};
use core::ffi::c_char;
pub trait PluginParamsPrototype<'host>:
    ExtensionPrototype<'host, Base = clap_plugin_params>
{
    fn count(&self) -> u32;
    fn get_info(&self, param_index: u32) -> Option<&clap_param_info>;
    fn get_value(&self, param_id: clap_id) -> Option<f64>;
    /// Takes in a mutable reference to a slice to write to
    fn value_to_text(&self, param_id: clap_id, value: f64, dst: &mut [i8]) -> Option<usize> {
        println!("ValueToText default impl");
        if param_id >= self.count() {
            return None;
        }
        self.value_to_text_function(param_id)(value, dst)
    }
    fn value_to_text_function(&self, param_id: clap_id) -> fn(f64, &mut [i8]) -> Option<usize>;
    fn text_to_value(
        &self,
        param_id: clap_id,
        param_value_text: &PluginParameterValueText,
    ) -> Option<f64>;
    fn flush(
        &self,
        plugin: &Self::Parent,
        in_: &clap_input_events,
        out: &clap_output_events,
    ) -> Option<()>;
}
fn get_ext<'host, 'ext, P, E>(ptr: *const clap_plugin) -> Option<&'ext E>
where
    P: HasExtension<'host, clap_plugin_params, ExtensionType = E> + 'ext,
    E: PluginParamsPrototype<'host, Parent = P>,
    'host: 'ext,
{
    println!("PARAMS ACCESS");
    let plugin = unsafe { ptr.as_ref() }?;
    let parent = unsafe { (plugin.plugin_data as *const P).as_ref() }?;
    parent.get_params_extension()
}
unsafe extern "C" fn count<'host, P, E>(plugin_ptr: *const clap_plugin) -> u32
where
    P: HasExtension<'host, clap_plugin_params, ExtensionType = E>,
    E: PluginParamsPrototype<'host, Parent = P>,
{
    println!("PARAMS COUNT EXT");
    let Some(plugin) = get_ext::<P, E>(plugin_ptr) else {
        return 0;
    };
    plugin.count()
}
unsafe extern "C" fn get_info<'host, P, E>(
    plugin_ptr: *const clap_plugin,
    param_index: u32,
    param_info_ptr: *mut clap_param_info,
) -> bool
where
    P: HasExtension<'host, clap_plugin_params, ExtensionType = E>,
    E: PluginParamsPrototype<'host, Parent = P>,
{
    println!("PARAMS GET INFO EXT");
    let Some(plugin) = get_ext::<P, E>(plugin_ptr) else {
        return false;
    };
    if let Some(info) = plugin.get_info(param_index) {
        unsafe { core::ptr::copy_nonoverlapping(info, param_info_ptr, 1) };
        return true;
    }
    false
}
unsafe extern "C" fn get_value<'host, P, E>(
    plugin_ptr: *const clap_plugin,
    param_id: clap_id,
    out_value: *mut f64,
) -> bool
where
    P: HasExtension<'host, clap_plugin_params, ExtensionType = E>,
    E: PluginParamsPrototype<'host, Parent = P>,
{
    let Some(plugin) = get_ext::<P, E>(plugin_ptr) else {
        return false;
    };
    if let Some(value) = plugin.get_value(param_id) {
        unsafe { core::ptr::write(out_value, value) };
        return true;
    };
    false
}
unsafe extern "C" fn value_to_text<'host, P, E>(
    plugin_ptr: *const clap_plugin,
    param_id: clap_id,
    value: f64,
    out_buffer: *mut c_char,
    out_buffer_capacity: u32,
) -> bool
where
    P: HasExtension<'host, clap_plugin_params, ExtensionType = E>,
    E: PluginParamsPrototype<'host, Parent = P>,
{
    let Some(plugin) = get_ext::<P, E>(plugin_ptr) else {
        return false;
    };
    let dst = unsafe { core::slice::from_raw_parts_mut(out_buffer, out_buffer_capacity as usize) };
    if let Some(value_size) = plugin.value_to_text(param_id, value, dst) {
        return value_size <= out_buffer_capacity as usize;
    };
    false
}
unsafe extern "C" fn text_to_value<'host, P, E>(
    plugin_ptr: *const clap_plugin,
    param_id: clap_id,
    param_value_text: *const c_char,
    out_value: *mut f64,
) -> bool
where
    P: HasExtension<'host, clap_plugin_params, ExtensionType = E>,
    E: PluginParamsPrototype<'host, Parent = P>,
{
    println!("PARAMS TEXT TO VALUE");
    let Some(plugin) = get_ext::<P, E>(plugin_ptr) else {
        return false;
    };
    let param_value_text = unsafe { PluginParameterValueText::from_ptr(param_value_text) };
    if let Some(value) = plugin.text_to_value(param_id, param_value_text) {
        unsafe { core::ptr::write(out_value, value) };
        return true;
    };
    false
}
unsafe extern "C" fn flush<'host, P, E>(
    plugin_ptr: *const clap_plugin,
    in_: *const clap_input_events,
    out: *const clap_output_events,
) where
    P: HasExtension<'host, clap_plugin_params, ExtensionType = E>,
    E: PluginParamsPrototype<'host, Parent = P>,
{
    println!("PARAMS EXT FLUSH");
    let Some(parent) = (unsafe { plugin_ptr.as_ref() }) else {
        return;
    };
    let Some(parent) = (unsafe { (parent.plugin_data as *const P).as_ref() }) else {
        return;
    };
    let plugin = parent.get_extension();
    let Some((in_, out)) = (unsafe { in_.as_ref() }).zip(unsafe { out.as_ref() }) else {
        return;
    };
    plugin.flush(parent, in_, out).unwrap_or(())
}
pub const fn vtable<'host, P, E>() -> &'static clap_plugin_params
where
    P: HasExtension<'host, clap_plugin_params, ExtensionType = E>,
    E: PluginParamsPrototype<'host, Parent = P>,
{
    &clap_plugin_params {
        count: Some(count::<'host, P, E>),
        get_info: Some(get_info::<'host, P, E>),
        get_value: Some(get_value::<'host, P, E>),
        value_to_text: Some(value_to_text::<'host, P, E>),
        text_to_value: Some(text_to_value::<'host, P, E>),
        flush: Some(flush::<'host, P, E>),
    }
}
pub const fn extension_pointer<'host, P, E>() -> ProtoPtr<'host, E>
where
    P: HasExtension<'host, clap_plugin_params, ExtensionType = E>,
    E: PluginParamsPrototype<'host, Parent = P>,
{
    let vt = vtable::<P, E>() as *const _;
    ProtoPtr(vt, ::core::marker::PhantomData)
}
