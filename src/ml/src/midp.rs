//! example usage:
//! ```
//! let mut keyboard = Keyboard::new();
//! let midi = Midi::open("res/twinkle.mid");
//! 
//! for event in &midi
//! {
//!     event.apply(&mut keyboard);
//! 
//!     // delta time since last event
//!     //
//!     // for multiple events happening at the same time(ie. chords),
//!     // the first sent even will have a nonzero delta time, while
//!     // the suceeding events have a zero delta time, indicating
//!     // the events happen simultaneously
//!     std::thread:sleep(event.dt());
//!     
//!     // now keyboard has velocity for each key, etc.
//! }
//! ```

use std::{io::{ BufReader, Read }, time::Duration};
use std::fmt::Display;
use std::path::Path;
use std::fs::File;

use crate::key::{ Key, Velocity };

/// a use-case `.midi` file parser and iterator
///
/// only supports a single track, and midi events are lazy loaded
/// and partially supported(ie. only those relevant to the MAESTRO
/// dataset are implemented)
///
/// as such, the `Midi` object can only be forward-stepped
pub struct Midi
{
    /// from header: pulses per quarter note, used to calculate `self.dtpt` given
    /// the BPM
    ppq: u16,
    /// MIDI events left before end of track
    left: u32,
    /// delta time per (MIDI) tick
    ///
    /// note that tempo information is ignored, as this parser is designed for
    /// live performances from the MAESTRO dataset which are unknown to composition
    /// tempo
    dtpt: Duration,
    /// the remaining encoded `.midi` data for the singleton track in
    /// the file
    ///
    /// `MThd` and `MTrk` headers have already been parsed
    track: Stream<BufReader<File>>,
}

impl Midi
{
    /// opens and parses a MIDI file
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error>
    {
        let mut stream = Stream::open(path)?;

        // read header
        if &stream.readn::<u8, 4>()? != b"MThd"
        {
            return Err(Error::InvalidFileHeader)
        }
        // read header length
        if stream.read::<u32>()? != 6
        {
            return Err(Error::InvalidHeaderLength)
        }
        // read header data
        let format = stream.read::<u16>()?;
        // num tracks
        let num_tracks = stream.read::<u16>()?;
        // only supports one track
        if format != 0 || num_tracks != 1
        {
            return Err(Error::InvalidTracksAmount)
        }
        // pulses per quater note
        let ppq = stream.read::<u16>()?;
        if ppq & 0x8000 != 0
        {
            return Err(Error::InvalidTimingMode)
        }
        // singular tick represented as a `Duration`
        // MIDI standard is 120 BPM if not specified
        let dtpt = Duration::from_secs_f32(60.0 / (120.0 * ppq as f32));

        // track chunk header
        if &stream.readn::<u8, 4>()? != b"MTrk"
        {
            return Err(Error::InvalidTrackheader)
        }
        // track length is how many events are left
        let left = stream.read::<u32>()?;
        
        Ok(Self { ppq, left, dtpt, track: stream })
    }

    /// read the MIDI file header from the stream
    /// returns `()`
    fn read_header<R: Read>(stream: &mut Stream<R>)
    {

    }
}

impl Iterator for Midi
{
    type Item = (MidiEvent, Duration);

    fn next(&mut self) -> Option<Self::Item>
    {
        todo!()
    }
}