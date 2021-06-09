use std::time::Duration;

pub use event::{ Event, EventKind };
pub use error::{ Error, Result };

use crate::io::{ BigEndian, Stream, VarInt };
use crate::key::{ Note, Pedal };

mod error;
mod event;

pub struct MidiFile<S>
{
    /// latest status byte
    status: u8,
    /// number of tracks left
    tracks: i16,
    /// pulses per quarter note
    ppq: i16,
    /// duration per tick
    dpt: Duration,
    /// byte stream, to parse, from the `.midi` file
    stream: S,
}

impl<S: Stream<BigEndian>> MidiFile<S>
{
    pub fn new(mut stream: S) -> Result<Self>
    {
        use Error::Parse;

        if stream.parse::<[u8; 4]>()? != *b"MThd"               // chunk header - MThd
        {
            Err(Parse("Invalid file header(expected MThd)"))?
        }
        if stream.parse::<i32>()? != 6                          // chunk header - length
        {
            Err(Parse("Invalid header length(expected 6)"))?
        }
        let _format = stream.parse::<i16>()?;                   // midi format - 0, 1, 2
        let tracks = stream.parse::<i16>()?;                    // number of tracks
        let ppq = stream.parse::<i16>()?;                       // ticks per quarter note

        if ppq as i32 & 0x8000 != 0
        {
            Err(Parse("Invalid timing mode (SMPTE timecode not supported)"))?
        }
        let dpt = Duration::from_micros(120 / ppq as u64);      // default tempo - 120BPM

        let mut this = Self { stream, tracks, ppq, dpt, status: 0 };

        this.begin_track()?;                                    // first track chunk - MTrk

        Ok(this)
    }

    /// iterate over this MIDI track's events
    /// ```
    /// fn main() -> midi::Result<()>
    /// {
    ///     let mut piano = Keyboard::new();
    ///     let mut midi = Midi::open("fur_elise.mid")?;
    ///     
    ///     while let Some(event) = midi.next()?
    ///     {
    ///         event.apply(&mut piano);
    ///         
    ///         println!("{}", piano);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn next(&mut self) -> Result<Option<Event>>
    {
        use Error::Parse;

        // delta time
        let dt = self.stream.parse::<VarInt>()? as u32;

        // peek (maybe) new status
        if self.stream.peek::<u8>()? & 0x80 != 0
        {
            // consume peek
            self.status = self.stream.parse::<u8>()?;
        }

        // parse event code
        match self.status & 0xF0
        {
            // meta event
            0xF0 if matches!(self.status, 0xFF) =>
            {
                // meta event type
                let mty = self.stream.parse::<u8>()?;
                // length of the event's data
                let len = self.stream.parse::<VarInt>()?;

                match mty
                {
                    // end of track
                    0x2F =>
                    {
                        if len != 0
                        {
                            Err(Parse("Invalid EndOfTrack event length(expected 0)"))?
                        }
                        // fully read one track
                        self.tracks -= 1;

                        // done
                        if self.tracks <= 0
                        {
                            Ok(None)
                        }
                        else
                        {
                            // read next track
                            self.begin_track()?;
                            // continue to first event in new track
                            self.next()
                        }
                    },
                    // tempo
                    0x51 =>
                    {
                        if len != 3
                        {
                            Err(Parse("Invalid Tempo event length(expected 3)"))?
                        }

                        // 3 byte value
                        let raw = self.stream.parse::<[u8; 3]>()?;
                        // microseconds per quarter note
                        let mspqn = u32::from_be_bytes([0, raw[0], raw[1], raw[2]]);
                        // perform the conversion to BPM
                        self.dpt = Duration::from_micros(mspqn as u64 / self.ppq as u64);
                        // continue to next event in new track
                        self.next()
                    },
                    // any other event, which we don't care about but still
                    // need to read to completion
                    _ =>
                    {
                        // consume data
                        self.stream.seek(std::io::SeekFrom::Current(len as _))?;

                        // unsupported, continue to next event
                        self.next() 
                    }
                }
            }
            // sys-ex event
            0xF0 if matches!(self.status, 0xF0 | 0xF7) =>
            {
                // length of the event's data
                let len = self.stream.parse::<VarInt>()?;

                // don't care about this event, but still need to consume its
                // data
                self.stream.seek(std::io::SeekFrom::Current(len as _))?;

                // unsupported, continue to next event
                self.next() 
            }
            // channel event
            evt => match evt
            {
                // note off: note number, velocity
                0x80 =>
                {
                    let note = self.stream.parse::<u8>()?;
                    let _vel = self.stream.parse::<u8>()?;

                    // valid note on 88-key keyboard
                    if let Some(note) = Note::new(note)
                    {
                        Ok(Some(Event(self.dpt * dt, EventKind::Key(note, 0))))
                    }
                    // out of bounds
                    else
                    {
                        // unsupported, continue to next event
                        self.next()
                    }
                }
                // note on: note number, velocity
                0x90 =>
                {
                    let note = self.stream.parse::<u8>()?;
                    let vel = self.stream.parse::<u8>()?;

                    // valid note on 88-key keyboard
                    if let Some(note) = Note::new(note)
                    {
                        Ok(Some(Event(self.dpt * dt, EventKind::Key(note, vel))))
                    }
                    // out of bounds
                    else
                    {
                        // unsupported, continue to next event
                        self.next()
                    }
                }
                // controller value: num, val
                0xB0 =>
                {
                    let num = self.stream.parse::<u8>()?;
                    let val = self.stream.parse::<u8>()?;

                    match num
                    {
                        // sustain pedal
                        0x40 => Ok(Some(Event(self.dpt * dt, EventKind::Pedal(Pedal::Damper, val)))),
                        // soft pedal
                        0x43 => Ok(Some(Event(self.dpt * dt, EventKind::Pedal(Pedal::Soft, val)))),
                        // other... unsupported, continue to next event
                        _ => self.next(),
                    }
                }
                // note aftertouch, pitch bend
                0xA0 | 0xE0 =>
                {
                    // don't care about these events, but still need to read
                    // them to completion
                    let _arg1 = self.stream.parse::<u8>()?;
                    let _arg2 = self.stream.parse::<u8>()?;

                    // unsupported, continue to next event
                    self.next()
                }
                // program change, channel aftertouch
                0xC0 | 0xD0 =>
                {
                    // don't care about these events, but still need to read
                    // them to completion
                    let _arg1 = self.stream.parse::<u8>()?;

                    // unsupported, continue to next event
                    self.next()
                }
                // ???
                _ =>
                {
                    // unsupported, continue to next event
                    self.next()
                }              
            }
        }
    }

    /// begin parsing a new track
    fn begin_track(&mut self) -> Result<()>
    {
        use Error::*;

        // chunk header
        if self.stream.parse::<[u8; 4]>()? != *b"MTrk"
        {
            return Err(Parse("invalid track chunk header"))
        }
        let _len = self.stream.parse::<i32>()?;

        // reset status
        self.status = 0;

        Ok(())
    }
}

impl MidiFile<std::io::BufReader<std::fs::File>>
{
    /// open a MIDI file from its path
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self>
    {
        Self::new(std::io::BufReader::new(std::fs::File::open(path)?))
    }
}