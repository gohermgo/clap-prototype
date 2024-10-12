use super::ExtensionPrototype;
use clap_sys::{
    events::{clap_input_events, clap_output_events},
    ext::params::{clap_param_info, clap_plugin_params},
    id::clap_id,
    plugin::clap_plugin,
};
use core::ffi::{c_char, CStr};
pub trait PluginParamsPrototype<'host>:
    ExtensionPrototype<'host, Base = clap_plugin_params>
{
    const PARAM_COUNT: u32;
    fn count(&self) -> u32 {
        Self::PARAM_COUNT
    }
    fn get_info(&self, param_index: u32) -> Option<clap_param_info>;
    fn get_value(&self, param_id: clap_id) -> Option<f64>;
    /// Takes in a mutable reference to a slice to write to
    fn value_to_text(&self, param_id: clap_id, value: f64, dst: &mut [i8]) -> Option<usize> {
        if param_id >= Self::PARAM_COUNT {
            return None;
        }
        self.value_to_text_function(param_id)(value, dst)
    }
    fn value_to_text_function(&self, param_id: clap_id) -> fn(f64, &mut [i8]) -> Option<usize>;
    fn text_to_value(&self, param_id: clap_id, param_value_text: &CStr) -> Option<f64>;
    fn flush(
        &self,
        plugin: &clap_plugin,
        in_: &clap_input_events,
        out: &clap_output_events,
    ) -> Option<()>;
}
unsafe extern "C" fn count<'host, P>(plugin_ptr: *const clap_plugin) -> u32
where
    P: PluginParamsPrototype<'host>,
{
    P::from_raw_plugin(plugin_ptr)
        .map(|params| params.count())
        .unwrap_or_default()
}
unsafe extern "C" fn get_info<'host, P>(
    plugin_ptr: *const clap_plugin,
    param_index: u32,
    param_info_ptr: *mut clap_param_info,
) -> bool
where
    P: PluginParamsPrototype<'host>,
{
    P::from_raw_plugin(plugin_ptr)
        .and_then(|params| params.get_info(param_index))
        .map(|info| *param_info_ptr = info)
        .is_some()
}
unsafe extern "C" fn get_value<'host, P>(
    plugin_ptr: *const clap_plugin,
    param_id: clap_id,
    out_value: *mut f64,
) -> bool
where
    P: PluginParamsPrototype<'host>,
{
    P::from_raw_plugin(plugin_ptr)
        .and_then(|params| params.get_value(param_id))
        .map(|value| *out_value = value)
        .is_some()
}
unsafe extern "C" fn value_to_text<'host, P>(
    plugin_ptr: *const clap_plugin,
    param_id: clap_id,
    value: f64,
    out_buffer: *mut c_char,
    out_buffer_capacity: u32,
) -> bool
where
    P: PluginParamsPrototype<'host>,
{
    P::from_raw_plugin(plugin_ptr)
        .and_then(|params| {
            params.value_to_text(param_id, value, unsafe {
                core::slice::from_raw_parts_mut(out_buffer, out_buffer_capacity as usize)
            })
        })
        .map(|bytes_written| bytes_written <= out_buffer_capacity as usize)
        .is_some_and(core::convert::identity)
}
unsafe extern "C" fn text_to_value<'host, P>(
    plugin_ptr: *const clap_plugin,
    param_id: clap_id,
    param_value_text: *const c_char,
    out_value: *mut f64,
) -> bool
where
    P: PluginParamsPrototype<'host>,
{
    P::from_raw_plugin(plugin_ptr)
        .and_then(|params| params.text_to_value(param_id, CStr::from_ptr(param_value_text)))
        .map(|value| *out_value = value)
        .is_some()
}
unsafe extern "C" fn flush<'host, P>(
    plugin_ptr: *const clap_plugin,
    in_: *const clap_input_events,
    out: *const clap_output_events,
) where
    P: PluginParamsPrototype<'host>,
{
    P::from_raw_plugin(plugin_ptr)
        .and_then(|params| params.flush(plugin_ptr.as_ref()?, in_.as_ref()?, out.as_ref()?));
}
pub const fn vtable<'host, P>() -> &'static clap_plugin_params
where
    P: PluginParamsPrototype<'host>,
{
    &clap_plugin_params {
        count: Some(count::<'host, P>),
        get_info: Some(get_info::<'host, P>),
        get_value: Some(get_value::<'host, P>),
        value_to_text: Some(value_to_text::<'host, P>),
        text_to_value: Some(text_to_value::<'host, P>),
        flush: Some(flush::<'host, P>),
    }
}
