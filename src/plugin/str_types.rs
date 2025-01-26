use core::ffi::c_char;
use core::mem::transmute;
use std::sync::Arc;

use clap_sys::string_sizes::{CLAP_NAME_SIZE, CLAP_PATH_SIZE};

pub trait PluginStringSlice {
    type Pointer;
    fn as_ptr(&self) -> Self::Pointer;
}
impl<T> PluginStringSlice for Arc<T>
where
    T: PluginStringSlice,
{
    type Pointer = T::Pointer;
    fn as_ptr(&self) -> Self::Pointer {
        T::as_ptr(self.as_ref())
    }
}
const fn to_fixed<const FIXED_LENGTH: usize>(data: *const i8, len: usize) -> [i8; FIXED_LENGTH] {
    let mut buf = [0_i8; FIXED_LENGTH];
    unsafe { core::ptr::copy(data, buf.as_mut_ptr(), len) };
    buf
}
macro_rules! string_component {
    ($t:tt) => {
        ::clap_proc_tools::ez_c_str! { $t }
        impl PluginStringSlice for $t {
            type Pointer = *const i8;
            fn as_ptr(&self) -> Self::Pointer {
                self.as_ref().as_ptr()
            }
        }
    };
}
string_component! { PluginParameterValueText }
string_component! { PluginID }
string_component! { PluginName }
impl PluginName {
    pub const fn to_fixed(&self) -> [i8; CLAP_NAME_SIZE] {
        to_fixed(self.0.as_ptr(), self.0.count_bytes())
    }
}
string_component! { PluginVendor }
string_component! { PluginURL }
string_component! { PluginVersion }
string_component! { PluginDescription }
string_component! { PluginPath }
impl PluginPath {
    pub const fn to_fixed(&self) -> [i8; CLAP_PATH_SIZE] {
        to_fixed(self.0.as_ptr(), self.0.count_bytes())
    }
}
string_component! { PluginExtensionID }

string_component! { PluginFeature }

#[repr(transparent)]
pub struct RawPluginFeature(*const c_char);
impl RawPluginFeature {
    pub const fn nth(&self, n: usize) -> Option<&c_char> {
        if n > CLAP_NAME_SIZE {
            return None;
        };

        match unsafe {
            let character_ptr = self.0.add(n);
            character_ptr.as_ref()
        } {
            Some(0) => None,
            Some(c) => Some(c),
            _ => None,
        }
    }
    pub const fn calculate_length(&self) -> Option<usize> {
        if self.0.is_null() {
            return None;
        }
        let mut characters_processed = 0;
        while self.nth(characters_processed).is_some() {
            characters_processed += 1;
        }
        if characters_processed == 0 {
            None
        } else {
            // Add 1, to catch the zero
            Some(characters_processed + 1)
        }
    }
    pub const fn from_ptr(ptr: *const c_char) -> Option<RawPluginFeature> {
        let this = RawPluginFeature(ptr);
        // SIDE-EFFECT: Validates length and chars
        if this.calculate_length().is_none() {
            return None;
        }
        Some(this)
    }
    pub const fn as_slice(&self) -> &PluginFeature {
        unsafe {
            let len = self.calculate_length().unwrap_unchecked();
            let slice = core::slice::from_raw_parts(self.0, len);
            transmute(slice)
        }
    }
}
impl AsRef<PluginFeature> for RawPluginFeature {
    fn as_ref(&self) -> &PluginFeature {
        self.as_slice()
    }
}
#[inline(always)]
fn feature_from_ptr<'desc>(ptr: *const c_char) -> Option<&'desc PluginFeature> {
    use core::mem::transmute;
    use core::ptr::slice_from_raw_parts;

    fn next_character<'desc>(
        base_ptr: *const c_char,
        characters_processed: usize,
    ) -> Option<&'desc c_char> {
        if characters_processed > CLAP_NAME_SIZE {
            return None;
        };
        unsafe {
            let character_ptr = base_ptr.add(characters_processed);
            character_ptr
                .as_ref()
                .filter(|character| character.ne(&&0_i8))
        }
    }

    if ptr.is_null() {
        return None;
    }
    let mut characters_processed = 0;
    while next_character(ptr, characters_processed).is_some() {
        characters_processed += 1;
    }
    if characters_processed == 0 {
        return None;
    }

    // Ensure we include the null-byte
    let zero_index = characters_processed + 1;

    let ptr_slice = slice_from_raw_parts(ptr, zero_index);

    unsafe {
        let feature: *const PluginFeature = transmute(ptr_slice);
        feature.as_ref()
    }
}
#[repr(transparent)]
pub struct RawPluginFeatureSet(*const *const c_char);
impl RawPluginFeatureSet {
    pub const fn nth(&self, n: usize) -> Option<RawPluginFeature> {
        match unsafe {
            // Offset the base by rows, incrementing
            // a *const i8-sized unit per row-offset,
            // effectively offsetting to the next feature
            let row_base = self.0.add(n);
            row_base.as_ref()
        } {
            Some(base_ptr) => RawPluginFeature::from_ptr(*base_ptr),
            _ => None,
        }
    }
    pub const fn calculate_length(&self) -> Option<usize> {
        if self.0.is_null() {
            return None;
        }
        let mut row = 0;

        // Traverse nested c-array,
        // count rows so we can safely
        // communicate to rustc how we
        // want to break it down
        while self.nth(row).is_some() {
            row += 1;
        }

        if row == 0 { None } else { Some(row) }
    }
    pub fn from_ptr(ptr: *const *const c_char) -> Option<RawPluginFeatureSet> {
        if RawPluginFeatureSet(ptr).calculate_length().is_none() {
            None
        } else {
            Some(unsafe { transmute(ptr) })
        }
    }
}
fn feature_slice_from_ptr<'desc>(ptr: *const *const i8) -> Option<&'desc [&'desc PluginFeature]> {
    let mut rows = 0;
    let mut characters = 0;

    fn next_feature<'host>(
        set_base: *const *const i8,
        row_offset: usize,
    ) -> Option<&'host PluginFeature> {
        // Offset the base by rows, incrementing
        // a *const i8-sized unit per row-offset,
        // effectively offsetting to the next feature
        let row_base = unsafe { set_base.add(row_offset).as_ref() }?;
        feature_from_ptr(*row_base)
    }

    while let Some(_) = next_feature(ptr, rows) {
        rows += 1;
    }

    let slice: &[*const i8] = unsafe { core::slice::from_raw_parts(ptr, rows) };
    let slice: &'desc [&'desc PluginFeature] = unsafe { core::mem::transmute(slice) };
    Some(slice)
}
