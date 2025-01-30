//! The GUI-extension
//!
//! This extension defines how the plugin will present its GUI.
//!
//! There are two approaches:
//! 1. the plugin creates a window and embeds it into the host's window
//! 2. the plugin creates a floating window
//!
//! Embedding the window gives more control to the host, and feels more integrated.
//! Floating window are sometimes the only option due to technical limitations.
//!
//! Showing the GUI works as follow:
//!  1. clap_plugin_gui->is_api_supported(), check what can work
//!  2. clap_plugin_gui->create(), allocates gui resources
//!  3. if the plugin window is floating
//!  4.    -> clap_plugin_gui->set_transient()
//!  5.    -> clap_plugin_gui->suggest_title()
//!  6. else
//!  7.    -> clap_plugin_gui->set_scale()
//!  8.    -> clap_plugin_gui->can_resize()
//!  9.    -> if resizable and has known size from previous session, clap_plugin_gui->set_size()
//! 10.    -> else clap_plugin_gui->get_size(), gets initial size
//! 11.    -> clap_plugin_gui->set_parent()
//! 12. clap_plugin_gui->show()
//! 13. clap_plugin_gui->hide()/show() ...
//! 14. clap_plugin_gui->destroy() when done with the gui
//!
//! Resizing the window (initiated by the plugin, if embedded):
//! 1. Plugins calls clap_host_gui->request_resize()
//! 2. If the host returns true the new size is accepted,
//!    the host doesn't have to call clap_plugin_gui->set_size().
//!    If the host returns false, the new size is rejected.
//!
//! Resizing the window (drag, if embedded)):
//! 1. Only possible if clap_plugin_gui->can_resize() returns true
//! 2. Mouse drag -> new_size
//! 3. clap_plugin_gui->adjust_size(new_size) -> working_size
//! 4. clap_plugin_gui->set_size(working_size)

use core::ffi::c_char;

use crate::ProtoPtr;

use crate::plugin::HasExtension;
use crate::plugin::{PluginGUIWindowAPIName, PluginGUIWindowTitle};

use crate::ext::ExtensionPrototype;

use clap_sys::plugin::clap_plugin;

use clap_sys::ext::gui::clap_plugin_gui;
use clap_sys::ext::gui::{clap_gui_resize_hints, clap_window};

pub enum PluginGUIWindowAPI {
    Windows,
    Cocoa,
    X11,
    Wayland,
}
impl PluginGUIWindowAPI {
    pub fn maybe_from(api_name: &PluginGUIWindowAPIName) -> Option<PluginGUIWindowAPI> {
        use clap_sys::ext::gui::{
            CLAP_WINDOW_API_COCOA, CLAP_WINDOW_API_WAYLAND, CLAP_WINDOW_API_WIN32,
            CLAP_WINDOW_API_X11,
        };
        match api_name {
            api_name if api_name == CLAP_WINDOW_API_WIN32 => Some(PluginGUIWindowAPI::Windows),
            api_name if api_name == CLAP_WINDOW_API_COCOA => Some(PluginGUIWindowAPI::Cocoa),
            api_name if api_name == CLAP_WINDOW_API_WAYLAND => Some(PluginGUIWindowAPI::Wayland),
            api_name if api_name == CLAP_WINDOW_API_X11 => Some(PluginGUIWindowAPI::X11),
            _ => None,
        }
    }
}
impl From<PluginGUIWindowAPI> for &'static PluginGUIWindowAPIName {
    fn from(value: PluginGUIWindowAPI) -> Self {
        use clap_sys::ext::gui::{
            CLAP_WINDOW_API_COCOA, CLAP_WINDOW_API_WAYLAND, CLAP_WINDOW_API_WIN32,
            CLAP_WINDOW_API_X11,
        };
        match value {
            PluginGUIWindowAPI::Windows => {
                PluginGUIWindowAPIName::from_c_str(CLAP_WINDOW_API_WIN32)
            }
            PluginGUIWindowAPI::Cocoa => PluginGUIWindowAPIName::from_c_str(CLAP_WINDOW_API_COCOA),
            PluginGUIWindowAPI::Wayland => {
                PluginGUIWindowAPIName::from_c_str(CLAP_WINDOW_API_WAYLAND)
            }
            PluginGUIWindowAPI::X11 => PluginGUIWindowAPIName::from_c_str(CLAP_WINDOW_API_X11),
        }
    }
}

pub struct PluginGUIWindowAPIDetails {
    pub window_api: PluginGUIWindowAPI,
    pub is_floating: bool,
}

#[repr(C)]
pub struct PluginGUIWindowSize {
    pub width: u32,
    pub height: u32,
}

pub trait PluginGUIPrototype<'host>: ExtensionPrototype<'host, Base = clap_plugin_gui> {
    /// `main-thread`
    ///
    /// Returns true if the requested gui api is supported
    fn is_api_supported(&self, window_details: PluginGUIWindowAPIDetails) -> bool;
    /// `main-thread`
    ///
    /// Returns true if the plugin has a preferred api.
    /// The host has no obligation to honor the plugin preference, this is just a hint.
    fn get_preferred_api(&self) -> Option<PluginGUIWindowAPIDetails>;
    /// `main-thread`
    ///
    /// Create and allocate all resources necessary for the gui.
    ///
    /// If is_floating is true, then the window will not be managed by the host. The plugin
    /// can set its window to stays above the parent window, see set_transient().
    /// api may be null or blank for floating window.
    ///
    /// If is_floating is false, then the plugin has to embed its window into the parent window, see
    /// set_parent().
    ///
    /// After this call, the GUI may not be visible yet; don't forget to call show().
    ///
    /// Returns true if the GUI is successfully created.
    fn create(&self, window_details: PluginGUIWindowAPIDetails) -> bool;
    /// `main-thread`
    ///
    /// Free all resources associated with the gui.
    fn destroy(&self);
    /// `main-thread`
    ///
    /// Set the absolute GUI scaling factor, and override any OS info.
    /// Should not be used if the windowing api relies upon logical pixels.
    ///
    /// If the plugin prefers to work out the scaling factor itself by querying the OS directly,
    /// then ignore the call.
    ///
    /// scale = 2 means 200% scaling.
    ///
    /// Returns true if the scaling could be applied
    /// Returns false if the call was ignored, or the scaling could not be applied.
    fn set_scale(&self, scale: f64) -> bool;
    /// `main-thread`
    ///
    /// Get the current size of the plugin UI.
    /// clap_plugin_gui->create() must have been called prior to asking the size.
    ///
    /// Returns true if the plugin could get the size.
    fn get_size(&self) -> Option<PluginGUIWindowSize>;
    /// `main-thread` & `!floating`
    ///
    /// Returns true if the window is resizeable (mouse drag).
    fn can_resize(&self) -> bool;
    /// `main-thread` & `!floating`
    ///
    /// Returns true if the plugin can provide hints on how to resize the window.
    fn get_resize_hints(&self) -> Option<clap_gui_resize_hints>;
    /// `main-thread` & `!floating`
    ///
    /// If the plugin gui is resizable, then the plugin will calculate the closest
    /// usable size which fits in the given size.
    /// This method does not change the size.
    ///
    /// Returns true if the plugin could adjust the given size.
    fn adjust_size(&self, given_size: &mut PluginGUIWindowSize) -> bool;
    /// `main-thread` & `!floating`
    ///
    /// Sets the window size.
    ///
    /// Returns true if the plugin could resize its window to the given size.
    fn set_size(&self, target_size: PluginGUIWindowSize) -> bool;
    /// `main-thread` & `!floating`
    ///
    /// Embeds the plugin window into the given window.
    ///
    /// Returns true on success.
    fn set_parent(&self, window: &'host clap_window) -> bool;
    /// `main-thread` & `floating`
    /// Set the plugin floating window to stay above the given window.
    ///
    /// Returns true on success.
    fn set_transient(&self, window: &'host clap_window) -> bool;
    /// `main-thread` & `floating`
    ///
    /// Suggests a window title. Only for floating windows.
    fn suggest_title(&self, title: &PluginGUIWindowTitle);
    /// `main-thread`
    ///
    /// Show the window.
    ///
    /// Returns true on success.
    fn show(&self) -> bool;
    /// `main-thread`
    ///
    /// Hide the window, this method does not free the resources, it just hides
    /// the window content. Yet it may be a good idea to stop painting timers.
    ///
    /// Returns true on success.
    fn hide(&self) -> bool;
}
fn get_ext<'host, 'ext, P, E>(plugin: *const clap_plugin) -> Option<&'ext E>
where
    P: HasExtension<'host, clap_plugin_gui, ExtensionType = E> + 'ext,
    E: PluginGUIPrototype<'host, Base = clap_plugin_gui>,
    'host: 'ext,
{
    let plugin = unsafe { plugin.as_ref() }?;
    let parent = unsafe { (plugin.plugin_data as *const P).as_ref() }?;
    Some(parent.get_extension())
}
unsafe extern "C" fn is_api_supported<'host, P, E>(
    plugin: *const clap_plugin,
    api: *const c_char,
    is_floating: bool,
) -> bool
where
    P: HasExtension<'host, clap_plugin_gui, ExtensionType = E>,
    E: PluginGUIPrototype<'host, Base = clap_plugin_gui>,
{
    println!("GUI_IS_API_SUPPORTED");
    let Some(ext) = get_ext::<P, E>(plugin) else {
        return false;
    };
    let name = unsafe { PluginGUIWindowAPIName::from_ptr(api) };
    let Some(window_api) = PluginGUIWindowAPI::maybe_from(name) else {
        println!("What API is {name:?} tf");
        return false;
    };
    ext.is_api_supported(PluginGUIWindowAPIDetails {
        window_api,
        is_floating,
    })
}
unsafe extern "C" fn get_preferred_api<'host, P, E>(
    plugin: *const clap_plugin,
    api: *mut *const c_char,
    out_is_floating: *mut bool,
) -> bool
where
    P: HasExtension<'host, clap_plugin_gui, ExtensionType = E>,
    E: PluginGUIPrototype<'host, Base = clap_plugin_gui>,
{
    println!("GUI_GET_PREFERRED_API");
    let Some(ext) = get_ext::<P, E>(plugin) else {
        return false;
    };
    let Some(PluginGUIWindowAPIDetails {
        window_api,
        is_floating: is_floating_window,
    }) = ext.get_preferred_api()
    else {
        println!("no preferred api returned");
        return false;
    };
    let name: &'static PluginGUIWindowAPIName = window_api.into();
    unsafe {
        *api = name.as_ptr();
        *out_is_floating = is_floating_window;
    };
    true
}
unsafe extern "C" fn create<'host, P, E>(
    plugin: *const clap_plugin,
    api: *const c_char,
    in_is_floating: bool,
) -> bool
where
    P: HasExtension<'host, clap_plugin_gui, ExtensionType = E>,
    E: PluginGUIPrototype<'host, Base = clap_plugin_gui>,
{
    println!("GUI_CREATE");
    let Some(ext) = get_ext::<P, E>(plugin) else {
        return false;
    };
    let name = unsafe { PluginGUIWindowAPIName::from_ptr(api) };
    let Some(window_api) = PluginGUIWindowAPI::maybe_from(name) else {
        println!("What API is {name:?} tf");
        return false;
    };
    ext.create(PluginGUIWindowAPIDetails {
        window_api,
        is_floating: in_is_floating,
    })
}
unsafe extern "C" fn destroy<'host, P, E>(plugin: *const clap_plugin)
where
    P: HasExtension<'host, clap_plugin_gui, ExtensionType = E>,
    E: PluginGUIPrototype<'host, Base = clap_plugin_gui>,
{
    println!("GUI_DESTROY");
    let Some(ext) = get_ext::<P, E>(plugin) else {
        return;
    };
    ext.destroy();
}
unsafe extern "C" fn set_scale<'host, P, E>(plugin: *const clap_plugin, scale: f64) -> bool
where
    P: HasExtension<'host, clap_plugin_gui, ExtensionType = E>,
    E: PluginGUIPrototype<'host, Base = clap_plugin_gui>,
{
    println!("GUI_SET_SCALE");
    let Some(ext) = get_ext::<P, E>(plugin) else {
        return false;
    };
    ext.set_scale(scale)
}
unsafe extern "C" fn get_size<'host, P, E>(
    plugin: *const clap_plugin,
    out_width: *mut u32,
    out_height: *mut u32,
) -> bool
where
    P: HasExtension<'host, clap_plugin_gui, ExtensionType = E>,
    E: PluginGUIPrototype<'host, Base = clap_plugin_gui>,
{
    println!("GUI_GET_SIZE");
    let Some(ext) = get_ext::<P, E>(plugin) else {
        return false;
    };
    let Some(PluginGUIWindowSize { width, height }) = ext.get_size() else {
        println!("Could not get size");
        return false;
    };

    unsafe {
        *out_width = width;
        *out_height = height;
    };

    true
}
unsafe extern "C" fn can_resize<'host, P, E>(plugin: *const clap_plugin) -> bool
where
    P: HasExtension<'host, clap_plugin_gui, ExtensionType = E>,
    E: PluginGUIPrototype<'host, Base = clap_plugin_gui>,
{
    println!("GUI_GET_SIZE");
    let Some(ext) = get_ext::<P, E>(plugin) else {
        return false;
    };
    ext.can_resize()
}
unsafe extern "C" fn get_resize_hints<'host, P, E>(
    plugin: *const clap_plugin,
    out_hints: *mut clap_gui_resize_hints,
) -> bool
where
    P: HasExtension<'host, clap_plugin_gui, ExtensionType = E>,
    E: PluginGUIPrototype<'host, Base = clap_plugin_gui>,
{
    println!("GUI_GET_RESIZE_HINTS");
    let Some(ext) = get_ext::<P, E>(plugin) else {
        return false;
    };
    let Some(hints) = ext.get_resize_hints() else {
        println!("Failed to get hints");
        return false;
    };
    unsafe {
        *out_hints = hints;
    }
    true
}
unsafe extern "C" fn adjust_size<'host, P, E>(
    plugin: *const clap_plugin,
    width: *mut u32,
    height: *mut u32,
) -> bool
where
    P: HasExtension<'host, clap_plugin_gui, ExtensionType = E>,
    E: PluginGUIPrototype<'host, Base = clap_plugin_gui>,
{
    println!("GUI_ADJUST_SIZE");
    let Some(ext) = get_ext::<P, E>(plugin) else {
        return false;
    };
    let mut window_size: PluginGUIWindowSize = PluginGUIWindowSize {
        width: unsafe { *width },
        height: unsafe { *height },
    };
    if ext.adjust_size(&mut window_size) {
        unsafe {
            *width = window_size.width;
            *height = window_size.height;
        }
        true
    } else {
        false
    }
}
unsafe extern "C" fn set_size<'host, P, E>(
    plugin: *const clap_plugin,
    width: u32,
    height: u32,
) -> bool
where
    P: HasExtension<'host, clap_plugin_gui, ExtensionType = E>,
    E: PluginGUIPrototype<'host, Base = clap_plugin_gui>,
{
    println!("GUI_SET_SIZE");
    let Some(ext) = get_ext::<P, E>(plugin) else {
        return false;
    };
    ext.set_size(PluginGUIWindowSize { width, height })
}
unsafe extern "C" fn set_parent<'host, P, E>(
    plugin: *const clap_plugin,
    window: *const clap_window,
) -> bool
where
    P: HasExtension<'host, clap_plugin_gui, ExtensionType = E>,
    E: PluginGUIPrototype<'host, Base = clap_plugin_gui>,
{
    println!("GUI_SET_PARENT");
    let Some(ext) = get_ext::<P, E>(plugin) else {
        return false;
    };
    let Some(parent) = (unsafe { window.as_ref() }) else {
        println!("parent null");
        return false;
    };
    ext.set_parent(parent)
}
unsafe extern "C" fn set_transient<'host, P, E>(
    plugin: *const clap_plugin,
    window: *const clap_window,
) -> bool
where
    P: HasExtension<'host, clap_plugin_gui, ExtensionType = E>,
    E: PluginGUIPrototype<'host, Base = clap_plugin_gui>,
{
    println!("GUI_SET_TRANSIENT");
    let Some(ext) = get_ext::<P, E>(plugin) else {
        return false;
    };
    let Some(transient) = (unsafe { window.as_ref() }) else {
        println!("transient null");
        return false;
    };
    ext.set_transient(transient)
}
unsafe extern "C" fn suggest_title<'host, P, E>(plugin: *const clap_plugin, title: *const c_char)
where
    P: HasExtension<'host, clap_plugin_gui, ExtensionType = E>,
    E: PluginGUIPrototype<'host, Base = clap_plugin_gui>,
{
    println!("GUI_SUGGEST_TITLE");
    let Some(ext) = get_ext::<P, E>(plugin) else {
        return;
    };
    let title = unsafe { PluginGUIWindowTitle::from_ptr(title) };
    ext.suggest_title(title);
}
unsafe extern "C" fn show<'host, P, E>(plugin: *const clap_plugin) -> bool
where
    P: HasExtension<'host, clap_plugin_gui, ExtensionType = E>,
    E: PluginGUIPrototype<'host, Base = clap_plugin_gui>,
{
    println!("GUI_SHOW");
    let Some(ext) = get_ext::<P, E>(plugin) else {
        return false;
    };
    ext.show()
}
unsafe extern "C" fn hide<'host, P, E>(plugin: *const clap_plugin) -> bool
where
    P: HasExtension<'host, clap_plugin_gui, ExtensionType = E>,
    E: PluginGUIPrototype<'host, Base = clap_plugin_gui>,
{
    println!("GUI_HIDE");
    let Some(ext) = get_ext::<P, E>(plugin) else {
        return false;
    };
    ext.hide()
}
pub const fn vtable<'host, P, E>() -> &'static clap_plugin_gui
where
    P: HasExtension<'host, clap_plugin_gui, ExtensionType = E>,
    E: PluginGUIPrototype<'host, Base = clap_plugin_gui>,
{
    &clap_plugin_gui {
        is_api_supported: Some(is_api_supported::<'host, P, E>),
        get_preferred_api: Some(get_preferred_api::<'host, P, E>),
        create: Some(create::<'host, P, E>),
        destroy: Some(destroy::<'host, P, E>),
        set_scale: Some(set_scale::<'host, P, E>),
        get_size: Some(get_size::<'host, P, E>),
        can_resize: Some(can_resize::<'host, P, E>),
        get_resize_hints: Some(get_resize_hints::<'host, P, E>),
        adjust_size: Some(adjust_size::<'host, P, E>),
        set_size: Some(set_size::<'host, P, E>),
        set_parent: Some(set_parent::<'host, P, E>),
        set_transient: Some(set_transient::<'host, P, E>),
        suggest_title: Some(suggest_title::<'host, P, E>),
        show: Some(show::<'host, P, E>),
        hide: Some(hide::<'host, P, E>),
    }
}
pub const fn extension_pointer<'host, P, E>() -> ProtoPtr<'host, E>
where
    P: HasExtension<'host, clap_plugin_gui, ExtensionType = E>,
    E: PluginGUIPrototype<'host, Base = clap_plugin_gui, Parent = P>,
{
    let vt = vtable::<P, E>() as *const _;
    ProtoPtr(vt, ::core::marker::PhantomData)
}
