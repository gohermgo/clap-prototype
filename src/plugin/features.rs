use core::borrow::Borrow;
use core::ffi::CStr;
use std::sync::Arc;

use crate::plugin::str_types::PluginFeature;

pub enum PluginFeatureKind {
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
impl From<PluginFeatureKind> for Arc<PluginFeature> {
    #[inline]
    fn from(value: PluginFeatureKind) -> Self {
        PluginFeature::from_c_str(value.as_c_str()).into()
    }
}
impl PluginFeatureKind {
    fn as_c_str(&self) -> &'static CStr {
        use PluginFeatureKind::*;
        use clap_sys::plugin_features::*;
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
impl Borrow<CStr> for PluginFeatureKind {
    fn borrow(&self) -> &CStr {
        self.as_c_str()
    }
}
