use crate::{AbstractPrototype, ProtoPtr};

pub mod audio_ports;
pub mod gui;
pub mod params;
pub mod state;
pub mod state_context;

pub use clap_proc_tools::extends;

pub trait ExtensionPrototype<'host>: AbstractPrototype<'host> {
    type Parent: AbstractPrototype<'host>;
}
