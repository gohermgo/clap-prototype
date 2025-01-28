pub(crate) mod descriptor;
pub mod features;
pub(crate) mod str_types;

use std::ptr::NonNull;

pub use descriptor::PluginDescriptor;
pub use str_types::*;

use crate::ext::ExtensionPointer;
use crate::ext::audio_ports::PluginAudioPortsPrototype;
use crate::ext::params::PluginParamsPrototype;
use crate::ext::state::PluginStatePrototype;
use crate::{AbstractPointer, AbstractPrototype};

use clap_sys::events::clap_output_events;
use clap_sys::plugin::clap_plugin;
use clap_sys::string_sizes::CLAP_NAME_SIZE;

pub trait PluginPrototype<'host>: AbstractPrototype<'host, Base = clap_plugin> {
    type PluginParamsExtension: PluginParamsPrototype<'host>;
    type PluginAudioPortsExtension: PluginAudioPortsPrototype<'host>;
    type PluginStateExtension: PluginStatePrototype<'host>;
    /// Sorry had to
    fn initialize(&mut self) -> bool;
    fn get_descriptor(&self) -> PluginDescriptor<'_>;
    fn get_id(&self) -> &PluginID {
        self.get_descriptor().id
    }
    fn sync_main_thread_with_audio_thread(&self, output_events: &clap_output_events) -> bool;
    fn sync_proc_thread_with_main_thread(&self) -> bool;
    fn get_params_extension(&self) -> Option<&Self::PluginParamsExtension> {
        None
    }
    fn get_audio_ports_extension(&self) -> Option<&Self::PluginAudioPortsExtension> {
        None
    }
    fn get_state_extension(&self) -> Option<&Self::PluginStateExtension> {
        None
    }
}

/// Work in progress trait based on my initial integration
pub trait ClapPlugin<'host> {
    type SyncSharedContainer<'shared>;
    type PluginData;
    type InitializationError;
    /// Function which needs precise definition.
    /// It will be called as part of the init-thunk
    fn static_initializer() -> impl FnOnce() -> Result<Self::PluginData, Self::InitializationError>;
    /// Function which needs precise definition.
    /// It will be called as part of the init-thunk
    /// At this point, the SyncShared container should be valid
    fn init_plugin(&mut self, this: &Self::PluginData) -> Option<bool>;
    /*
    fn get_init_thunk(
        &self,
        static_initializer: impl FnOnce() -> Result<Self::PluginData, Self::InitializationError>,
    ) -> Result<impl FnOnce(fn() -> &'host mut Self) -> Option<impl FnOnce() -> >, Self::InitializationError> {
        match static_initializer() {
            Ok(mut this) =>  Ok(|f: fn() -> &'host mut Self| {
                let zelf = f();
                zelf.init_plugin(this);
                || {
                    LazyLock::new(|| f)
                }
            }),
            Err(e) => Err(e),
        }
    }
    */
}
// type ParamValue = f64;
// type ParamFlag = bool;
// struct FlagParam {
//     changed: bool,
//     value: f64,
// }
/// Decision made to go with multi-array to hopefully minimize cost of
/// padding and cache-trampling (8 bytes vs. 1 byte...)
// struct FlagParamMultiArray<const C: usize> {
//     values: [ParamValue; C],
//     flags: [ParamFlag; C],
// }
// impl<const C: usize> FlagParamMultiArray<C> {
//     const fn has_changed(&self, idx: usize) -> bool {
//         assert!(idx < C, "OUT OF BOUNDS IDIOT");
//         self.flags[idx]
//     }
//     const fn new(default_value: f64) -> Self {
//         Self {
//             values: [default_value; C],
//             flags: [false; C],
//         }
//     }
//     fn write(&mut self, value: f64, idx: usize) {
//         self.flags[idx] = true;
//         self.values[idx] = value;
//     }
//     fn set_all_flags(&mut self) {
//         println!("FlagParamMultiArray:set-all-flags");
//         self.flags.iter_mut().for_each(|flag| *flag = true);
//     }
// }
/*
fn make_parameter_mutex<const C: usize>(resolve_param_fn: fn(i32)) -> Mutex<FlagParamMultiArray<C>> {
    Mutex::new(
        const {
            let mut buf = FlagParamMultiArray::new(0.);
            let mut idx = 0;
            while idx < C {
                buf.values[idx] = if idx == ext::params::distortion::PARAM_INFO.id as usize {
                    ext::params::distortion::PARAM_INFO.default_value
                } else {
                    todo!()
                };
                idx += 1;
            }
            buf
        },
    )
}
*/
// /// Structure containing one lock per 'threaded' storage
// /// as they are inherently linked.
// /// One thing to take into consideration however is the possibility
// /// of locking out process-critical writes.
// ///
// /// This should be taken care of by monitoring the activity state
// /// of the plugin
// ///     Active   => plugin is in processing function
// ///     Inactive => we have some time, acquire the lock, and update
// ///                 if needed
// ///
// /// This approach does give more flexibility regarding lock-lifetime,
// /// in that the RAII-nature of it allows us to define certain
// /// critical sections
// #[repr(C)]
// struct LinkedFlagParamMutex<const C: usize> {
//     main: Mutex<FlagParamMultiArray<C>>,
//     audio: Mutex<FlagParamMultiArray<C>>,
// }
/*
impl<const C: usize> LinkedFlagParamMutex<C> {
    fn new() -> Self {
        Self {
            main: make_parameter_mutex(),
            audio: make_parameter_mutex(),
        }
    }
}
*/
/// Wrap the double Mutex in a Mutex.
/// You want to write to either 'threaded' storage?
/// You will have to claim both, to ensure correct data transfer.
/// This indicates to any other thread attempting to access either,
/// that one (or both) is currently being read from/written to
/*
static PARAMETERS: LazyLock<Mutex<LinkedFlagParamMutex>> = LazyLock::new(|| {
    println!("There comes a time for every lazy-lock to init...");
    Mutex::new(LinkedFlagParamMutex::new())
});
pub trait InitializedClapPlugin<'host> {

}
*/

// /// Value semantics as-is, can of course be shoved in a
// /// Once/LazyLock, and forced into a static ref
// ///
// /// I found that that works pretty well
// pub struct ClapPluginWrapper<'host, Pg> {
//     this: Pg,
//     host: &'host clap_host,
// }
/*
impl<'host, P: ClapPlugin<'host>> ClapPlugin<'host> for ClapPluginWrapper<'host, P> {
    type SyncSharedContainer<'shared> = ();
    type PluginData = ;
    // pub fn new(pc_host: *const clap_host) -> Self {
    //     Self { this: (), host: () }
    // }
}
*/
// TODOOO BIG TODOOOOS
// I would love to proceed this, cramming some
// junk into here
//
// Could potentially make the implementation
// so simple, that I could run the validator
// against smaller units and bundles thereof
use clap_sys::plugin::clap_plugin_descriptor;
use clap_sys::version::clap_version;
// #[repr(C)]
// struct ClapPluginDescriptorBuilder {
//     version: Option<clap_version>,
//     id: Option<()>,
// }
#[repr(C)]
pub struct ClapPluginDescriptor {
    version: clap_version,
}
