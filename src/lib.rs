use clap_sys::process::{
    clap_process_status, CLAP_PROCESS_CONTINUE, CLAP_PROCESS_CONTINUE_IF_NOT_QUIET,
    CLAP_PROCESS_ERROR, CLAP_PROCESS_SLEEP, CLAP_PROCESS_TAIL,
};
use core::mem::transmute;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
pub mod ext;
pub mod factory;
pub mod plugin;
pub trait AbstractPrototype<'host> {
    type Base;
    // #[allow(clippy::missing_safety_doc)]
    // unsafe fn from_raw_base<'ptr>(base_ptr: *const Self::Base) -> Option<&'ptr Self>;
    fn as_base(&self) -> &Self::Base;
}
/// We force i8 representation here just to have some
/// Idea at this point:
///   -  In order to coerce all the Result-types to live in the same
///      enum, we have to pack the error-information somehow into a
///      smaller space.
///   -  Luckily, we know that CLAP maps error to 0, meaning we have
///      31 free bits to twiddle if we follow the MSFT example of
///      HRESULT fail indicating negative numbers.
///   -  Thus, I suggest the encoding:
///
///
#[repr(i8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntityKind {
    Plugin = 0b00,
    Extension = 0b01,
    Host = 0b10,
    Factory = 0b11,
    // This can continue, but based on my current idea,
    // no further tha 0xFF
}
impl From<u8> for EntityKind {
    fn from(value: u8) -> Self {
        if value > 0b11 {
            panic!("What are you doing thats like a WAY too big value. muh invariants");
        }
        // SAFETY: Look above. Explicit panic xd
        unsafe { transmute(value) }
    }
}
impl From<i8> for EntityKind {
    fn from(value: i8) -> Self {
        if value < 0b00 {
            panic!("THATS NEGATIVE BRUH IM USING THOSE NUMBERS");
        }
        EntityKind::from(value as u8)
    }
}
type ClapErrorKindCode = clap_process_status;
pub const CLAP_ERROR_KIND_NOT_FOUND: ClapErrorKindCode = 0b00;
pub const CLAP_ERROR_INIT_FAILED: ClapErrorKindCode = 0b01;
#[repr(i8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClapErrorKind {
    NotFound(EntityKind) = 0b00,
    InitFailed(EntityKind) = 0b01,
}
// impl From<u16> for ClapErrorKind {
//     fn from(value: u16) -> Self {
//         let (hi, lo) = ((value & 0xFF00 >> 8) as u8, (value & 0x00FF) as u8);
//         let ek = EntityKind::from(lo);

//     }
// }
// impl ClapErrorKind {
//     fn pack_to_dword(self) -> i16 {
//         todo!()
//     }
// }
#[repr(i32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProcessResult {
    /// Keep going
    Continue = CLAP_PROCESS_CONTINUE,
    ContinueIfNeeded = CLAP_PROCESS_CONTINUE_IF_NOT_QUIET,
    Sleep = CLAP_PROCESS_SLEEP,
    Tail = CLAP_PROCESS_TAIL,
    /// Error type, just wait and see this is gonna be sweet
    /// Might look like an anti-pattern at this point, but this way
    /// the API will be closer-ish to Rust/C FFI shit
    Err(Option<ClapErrorKind>) = CLAP_PROCESS_ERROR,
}
impl From<clap_process_status> for ProcessResult {
    fn from(value: clap_process_status) -> Self {
        match value {
            CLAP_PROCESS_CONTINUE => Self::Continue,
            CLAP_PROCESS_CONTINUE_IF_NOT_QUIET => Self::ContinueIfNeeded,
            CLAP_PROCESS_SLEEP => Self::Sleep,
            CLAP_PROCESS_TAIL => Self::Tail,
            CLAP_PROCESS_ERROR => todo!(),
            _ => panic!("WHO TF RETURNED A PROCES STATUS NOT DEFINED BY CLAP"),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
