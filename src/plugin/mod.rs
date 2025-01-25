use super::AbstractPrototype;
use clap_sys::plugin::clap_plugin;
use std::{borrow, ffi, ptr};
pub trait PluginDescriptorComponent {
    type Pointer;
    fn as_ptr(&self) -> Self::Pointer;
}
macro_rules! string_component {
    ($t:tt) => {
        #[repr(transparent)]
        pub struct $t(std::ffi::CString);
        impl std::borrow::Borrow<std::ffi::CStr> for $t {
            fn borrow(&self) -> &std::ffi::CStr {
                &self.0
            }
        }
        impl std::ops::Deref for $t {
            type Target = std::ffi::CString;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl AsRef<std::ffi::CStr> for $t {
            fn as_ref(&self) -> &std::ffi::CStr {
                std::borrow::Borrow::borrow(self)
            }
        }
        impl PluginDescriptorComponent for $t {
            type Pointer = *const i8;
            fn as_ptr(&self) -> Self::Pointer {
                self.0.as_ptr()
            }
        }
    };
}
pub trait FromRaw {
    type Raw;
    fn from_raw(raw: Self::Raw) -> Self;
}
string_component!(PluginID);
impl FromRaw for PluginID {
    type Raw = *const i8;
    fn from_raw(raw: Self::Raw) -> Self {
        let borrowed = unsafe { ffi::CStr::from_ptr(raw) };
        PluginID(borrow::ToOwned::to_owned(borrowed))
    }
}
#[repr(transparent)]
struct PluginIDSlice(ffi::CStr);
impl std::ops::Deref for PluginIDSlice {
    type Target = ffi::CStr;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
string_component!(PluginName);
string_component!(PluginVendor);
string_component!(PluginURL);
string_component!(PluginVersion);
string_component!(PluginDescription);
pub enum PluginFeature {
    Instrument,
    AudioEffect,
    NoteEffect,
    NoteDetector,
    Analyzer,
    Synthesizer,
    Sampler,
    Drum,
    DrumMachine,
    Filter,
    Phaser,
    Equalizer,
    DeEsser,
    PhaseVocoder,
    Granular,
    FrequencyShifter,
    PitchShifter,
    Distortion,
    TransientShaper,
    Compressor,
    Expander,
    Gate,
    Limiter,
    Flanger,
    Chorus,
    Delay,
    Reverb,
    Tremolo,
    Glitch,
    Utility,
    PitchCorrection,
    Restoration,
    MultiEffects,
    Mixing,
    Mastering,
    Mono,
    Stereo,
    Surround,
    Ambisonic,
}
impl PluginFeature {
    fn as_raw(&self) -> &'static ffi::CStr {
        use clap_sys::plugin_features::*;
        use PluginFeature::*;
        match self {
            Instrument => CLAP_PLUGIN_FEATURE_INSTRUMENT,
            AudioEffect => CLAP_PLUGIN_FEATURE_AUDIO_EFFECT,
            NoteEffect => CLAP_PLUGIN_FEATURE_NOTE_EFFECT,
            NoteDetector => CLAP_PLUGIN_FEATURE_NOTE_DETECTOR,
            Analyzer => CLAP_PLUGIN_FEATURE_ANALYZER,
            Synthesizer => CLAP_PLUGIN_FEATURE_SYNTHESIZER,
            Sampler => CLAP_PLUGIN_FEATURE_SAMPLER,
            Drum => CLAP_PLUGIN_FEATURE_DRUM,
            DrumMachine => CLAP_PLUGIN_FEATURE_DRUM_MACHINE,
            Filter => CLAP_PLUGIN_FEATURE_FILTER,
            Phaser => CLAP_PLUGIN_FEATURE_PHASER,
            Equalizer => CLAP_PLUGIN_FEATURE_EQUALIZER,
            DeEsser => CLAP_PLUGIN_FEATURE_DEESSER,
            PhaseVocoder => CLAP_PLUGIN_FEATURE_PHASE_VOCODER,
            Granular => CLAP_PLUGIN_FEATURE_GRANULAR,
            FrequencyShifter => CLAP_PLUGIN_FEATURE_FREQUENCY_SHIFTER,
            PitchShifter => CLAP_PLUGIN_FEATURE_PITCH_SHIFTER,
            Distortion => CLAP_PLUGIN_FEATURE_DISTORTION,
            TransientShaper => CLAP_PLUGIN_FEATURE_TRANSIENT_SHAPER,
            Compressor => CLAP_PLUGIN_FEATURE_COMPRESSOR,
            Expander => CLAP_PLUGIN_FEATURE_EXPANDER,
            Gate => CLAP_PLUGIN_FEATURE_GATE,
            Limiter => CLAP_PLUGIN_FEATURE_LIMITER,
            Flanger => CLAP_PLUGIN_FEATURE_FLANGER,
            Chorus => CLAP_PLUGIN_FEATURE_CHORUS,
            Delay => CLAP_PLUGIN_FEATURE_DELAY,
            Reverb => CLAP_PLUGIN_FEATURE_REVERB,
            Tremolo => CLAP_PLUGIN_FEATURE_TREMOLO,
            Glitch => CLAP_PLUGIN_FEATURE_GLITCH,
            Utility => CLAP_PLUGIN_FEATURE_UTILITY,
            PitchCorrection => CLAP_PLUGIN_FEATURE_PITCH_CORRECTION,
            Restoration => CLAP_PLUGIN_FEATURE_RESTORATION,
            MultiEffects => CLAP_PLUGIN_FEATURE_MULTI_EFFECTS,
            Mixing => CLAP_PLUGIN_FEATURE_MIXING,
            Mastering => CLAP_PLUGIN_FEATURE_MASTERING,
            Mono => CLAP_PLUGIN_FEATURE_MONO,
            Stereo => CLAP_PLUGIN_FEATURE_STEREO,
            Surround => CLAP_PLUGIN_FEATURE_SURROUND,
            Ambisonic => CLAP_PLUGIN_FEATURE_AMBISONIC,
        }
    }
}
impl borrow::Borrow<ffi::CStr> for PluginFeature {
    fn borrow(&self) -> &ffi::CStr {
        self.as_raw()
    }
}
impl PluginDescriptorComponent for PluginFeature {
    type Pointer = *const i8;
    fn as_ptr(&self) -> Self::Pointer {
        self.as_raw().as_ptr()
    }
}
pub struct PluginDescriptor {
    pub framework_version: clap_version,
    pub id: PluginID,
    pub name: PluginName,
    pub vendor: PluginVendor,
    pub url: PluginURL,
    pub manual_url: PluginURL,
    pub support_url: PluginURL,
    pub version: PluginVersion,
    pub description: PluginDescription,
    pub features: Vec<PluginFeature>,
}
impl PluginDescriptor {
    pub fn new() -> Self {}
    pub fn from_raw(raw: clap_plugin_descriptor) -> Self {
        Self {
            framework_version: raw.clap_version,
            id: PluginID::from_raw(raw.id),
        }
    }
    pub fn into_raw(self) -> clap_plugin_descriptor {
        let mut features: Vec<*const i8> = self
            .features
            .iter()
            .map(|feature| feature.as_raw().as_ptr())
            .collect();
        features.push(ptr::null());
        let features = features.as_ptr();
        clap_plugin_descriptor {
            clap_version: self.framework_version,
            id: self.id.as_ptr(),
            name: self.name.as_ptr(),
            vendor: self.vendor.as_ptr(),
            url: self.url.as_ptr(),
            manual_url: self.manual_url.as_ptr(),
            support_url: self.support_url.as_ptr(),
            version: self.version.as_ptr(),
            description: self.description.as_ptr(),
            features,
        }
    }
}
impl From<PluginDescriptor> for clap_plugin_descriptor {
    fn from(value: PluginDescriptor) -> Self {
        value.into_raw()
    }
}
pub trait PluginPrototype<'host>: AbstractPrototype<'host, Base = clap_plugin> {
    fn get_descriptor(&self) -> &PluginDescriptor;
    fn get_id(&self) -> &PluginID {
        &self.get_descriptor().id
    }
    // const DESCRIPTOR: &'static clap_plugin_descriptor;
    // const ID: &'static ffi::CStr = const { unsafe { ffi::CStr::from_ptr(Self::DESCRIPTOR.id) } };
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
