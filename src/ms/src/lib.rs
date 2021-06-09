pub use crate::key::{ Keyboard, Note, Octave, Tone, Semitone, Pedal };

pub mod mid;
pub mod wav;

mod key;
mod io;

#[cfg(test)]
mod test;