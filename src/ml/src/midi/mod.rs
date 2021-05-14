mod event;
mod error;
mod util;
mod key;

#[cfg(test)]
mod test;

pub use key::{ Keyboard, Note, Octave, Tone, Semitone };
pub use event::{ Event, EventKind };
pub use error::{ Result, Error };

use util::*;

use self::event::MidiEvent;

/// an iterator over a limited subset of the MIDI format specification
///
/// designed to parse `.midi` files found in the `maestro` dataset, and
/// not much more.
/// MIDI events are lazy-loaded, as in they remain encoded until iterated
/// over. Moreover, tracks are merged sequentially into one(played one after
/// the other seemlessly).
///
/// made following the specication from:
/// https://github.com/colxi/midi-parser-js/wiki/MIDI-File-Format-Specifications
pub struct Midi<R>
{
    /// MIDI file encoded data
    reader: R,

    /// tracks left
    left: u16,
    /// pulses per quarter note
    ppq: u16,
    /// duration per ticks
    dpt: std::time::Duration,
}

impl<R: std::io::Read + std::io::Seek> Midi<R>
{
    /// attempt to parse a MIDI file given a stream
    ///
    /// can fail for io reasons or parsing reasons, for which
    /// an error is returned
    pub fn new(mut reader: R) -> Result<Self>
    {
        /* read header chunk */
        if reader.decode::<[u8; 4]>()? != *b"MThd"  // chunk tag
        || reader.decode::<u32>()? != 6             // chunk len
        {
            return Err(Error::InvalidHeaderChunk)
        }
        let _format = reader.decode::<u16>()?;      // midi file format(0, 1, 2)
        let tracks = reader.decode::<u16>()?;       // number of tracks

        let ppq = match reader.decode::<u16>()?     // pulses per quarter note(ticks/beat)
        {
            // time division in ticks per beat
            n if n & 0x8000 == 0 => Ok(n),
            //  time division in frames per second
            _ => Err(Error::UnsupportedTimingMode),
        }?;
        // default tempo(120 BPM)
        let dpt = std::time::Duration::from_micros(120 / ppq as u64);

        /* read first track chunk */
        if reader.decode::<[u8; 4]>()? != *b"MTrk"  // chunk tag
        {
            return Err(Error::InvalidTrackChunk)
        }
        let _size = reader.decode::<u32>()?;        // track chunk len

        Ok(Self { reader, ppq, dpt, left: tracks })
    }

    // / iterate over this MIDI file's events by applying the relevant ones
    // / to the given keyboard state.
    // / 
    // / merges key/pedal events that happen simultaenously, and returns the
    // / non-zero delta time between the current event and the last
    // pub fn apply_iter<'a>(mut self, keyboard: &'a mut Keyboard)// -> impl Iterator<Item = (&'_ Keyboard, std::time::Duration)>
    // {
    //     //self.reader.decode::<Event>().unwrap();
    // }

    // pub fn step<'a>(&mut self, keyboard: &'a mut Keyboard) -> Result<Option<std::time::Duration>>
    // {

    // }
}

impl Midi<std::io::BufReader<std::fs::File>>
{
    /// open a MIDI file from its path
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self>
    {
        Self::new(std::io::BufReader::new(std::fs::File::open(path)?))
    }
}

impl<R: std::io::Read + std::io::Seek> Iterator for Midi<R>
{
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item>
    {
        loop
        {
            match self.reader.decode::<MidiEvent>().ok()?
            {
                MidiEvent::Keyboard(kind, dt) => break Some(Event(self.dpt * *dt, kind)),
                MidiEvent::Tempo(mspqn) =>
                {
                    self.dpt = std::time::Duration::from_micros(mspqn as u64 / self.ppq as u64)
                },
                MidiEvent::EndOfTrack =>
                {
                    self.left -= 1;
                    if self.left <= 0
                    {
                        break None;
                    }
                    /* read new track chunk */
                    if self.reader.decode::<[u8; 4]>().ok()? != *b"MTrk"    // chunk tag
                    {
                        // invalid track chunk tag!
                        break None;
                    }
                    let _size = self.reader.decode::<u32>().ok()?;          // track chunk len
                },
                MidiEvent::Unsupported => {/* do nothing */}
            }
        }
    }
}