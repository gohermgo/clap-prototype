use std::{
    borrow::Cow,
    ffi::{c_char, CStr},
    ops::Deref,
};

use clap_sys::plugin::clap_plugin;

#[repr(transparent)]
pub struct GUIApi<'api>(&'api [i8]);
impl<'api> GUIApi<'api> {
    #[inline(always)]
    pub const fn new(name: &'api CStr) -> Self {
        Self(unsafe { core::slice::from_raw_parts(name.as_ptr(), name.count_bytes()) })
    }
    #[inline(always)]
    pub const fn from_ptr(pc_name: *const c_char) -> Self {
        Self::new(unsafe { CStr::from_ptr(pc_name) })
    }
}
pub const GUI_API_WIN32: GUIApi = GUIApi::new(c"CLAP_WINDOW_API_WIN32");
impl<'api> Deref for GUIApi<'api> {
    type Target = &'api [i8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
/// Very early stage impl, need to think a lot about this,
/// this is the main hunk of the rendering processing/pipeline
///
/// Also these will be unsafe, I like the idea of writing the
/// plugin to the `reserved` plugin pointer/buffer
pub trait PluginGUIPrototype {
    const PREFERRED_IDX: usize;
    const SUPPORTED_APIS: &'static [GUIApi<'_>];
    type PluginImplementation;
    fn is_api_supported(
        &self,
        plugin: &Self::PluginImplementation,
        api: &GUIApi,
        is_floating: bool,
    ) -> bool {
        todo!()
    }
}

// unsafe extern "C" fn is_api_supported(
//     pc_plugin: *const clap_plugin,
//     api: *const c_char,
//     is_floating: bool,
// ) -> bool {
//     let (Some(r_plugin), gui_api) = (pc_plugin.as_ref(), GUIApi::from_ptr(api)) else {
//         return false;
//     };

//     println!(
//         "clap:ext:gui:is-api-supported:api={:?};is_floating={is_floating}",
//         gui_api.0
//     );

//     api_str == GUI_API_WIN32 && !is_floating
// }
