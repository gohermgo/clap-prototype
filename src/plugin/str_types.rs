use core::ffi::c_char;
use core::fmt::{Display, Formatter, Result as FmtResult};
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
string_component! { PluginVendor }
string_component! { PluginURL }
string_component! { PluginVersion }
string_component! { PluginDescription }
string_component! { PluginPath }
string_component! { PluginExtensionID }
string_component! { PluginFeature }
string_component! { PluginGUIWindowAPIName }
string_component! { PluginGUIWindowTitle }
#[derive(Debug)]
pub enum FromPtrError {
    Malformed(usize),
    MissingNul,
}

impl Display for FromPtrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let s = "Failed to convert from raw-pointer to plugin-path: ";
        match self {
            FromPtrError::Malformed(n) => f.write_fmt(format_args!(
                "{s} traversed {n} bytes before failing to dereference a byte"
            )),
            FromPtrError::MissingNul => f.write_fmt(format_args!("{s} unterminated string")),
        }
    }
}
fn try_from_inner<'s, T: ?Sized, const N: usize>(
    ptr: *const ::core::ffi::c_char,
    new: unsafe fn(*const ::core::ffi::c_char) -> &'s T,
) -> Result<&'s T, FromPtrError> {
    // Walk the pointer, we do not know if
    // the entire is valid
    for offset in 0..N {
        // If we can dereference safely the byte
        if let Some(val) = unsafe { ptr.byte_add(offset).as_ref() } {
            // Check if nul and early return
            if val == &0i8 {
                return Ok(unsafe { new(ptr) });
            }
        } else {
            // We could not deref, or find nul
            // so assume the string is malformed
            return Err(FromPtrError::Malformed(offset));
        }
    }
    // Tail return
    Err(FromPtrError::MissingNul)
}
impl TryFrom<*const ::core::ffi::c_char> for &PluginName {
    type Error = FromPtrError;
    fn try_from(value: *const ::core::ffi::c_char) -> Result<Self, Self::Error> {
        try_from_inner::<PluginName, CLAP_NAME_SIZE>(value, PluginName::from_ptr)
    }
}
impl PluginName {
    pub const fn to_fixed(&self) -> [i8; CLAP_NAME_SIZE] {
        to_fixed(self.0.as_ptr(), self.0.count_bytes())
    }
}
impl TryFrom<*const ::core::ffi::c_char> for &PluginPath {
    type Error = FromPtrError;
    fn try_from(value: *const ::core::ffi::c_char) -> Result<Self, Self::Error> {
        try_from_inner::<PluginPath, CLAP_PATH_SIZE>(value, PluginPath::from_ptr)
    }
}
impl PluginPath {
    pub const fn to_fixed(&self) -> [i8; CLAP_PATH_SIZE] {
        to_fixed(self.0.as_ptr(), self.0.count_bytes())
    }
}

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
    pub const fn from_ptr(ptr: *const *const c_char) -> Option<RawPluginFeatureSet> {
        if RawPluginFeatureSet(ptr).calculate_length().is_none() {
            None
        } else {
            Some(RawPluginFeatureSet(ptr))
        }
    }
    pub const fn as_slice(&self) -> &[RawPluginFeature] {
        let len = self.calculate_length();
        unsafe {
            let len = len.unwrap_unchecked();
            let ptr_slice = core::ptr::slice_from_raw_parts(self.0, len);
            let slice: *const [RawPluginFeature] = transmute(ptr_slice);
            slice.as_ref().unwrap_unchecked()
        }
    }
    pub fn as_ptr(&self) -> *const *const c_char {
        self.0
    }
}
