use std::io::{ Read, Seek };
use std::time::Duration;

use super::util::*;
use super::key::*;

/// a subset of events as specified by the MIDI specification,
/// designed for `.midi` files in the `maestro` dataset and not
/// much more. includes the delta time between this event and the
/// last, a value of 0 indicating the two events happen simultaneously.
pub struct Event(pub Duration, pub EventKind);

/// a subset of events as specified by the MIDI specification,
/// designed for `.midi` files in the `maestro` dataset and not
/// much more
pub enum EventKind
{
    /// represents some change to a key, giving the key affected
    /// and its press velocity or 0 if released
    Key(Note, u8),
    /// represents some change to a pedal, giving the pedal affected
    /// and its new press value
    Pedal(Pedal, u8),
}

/// `Event` + a few needed by `Midi`
pub(super) enum MidiEvent
{
    /// keyboard event to be passed to the top level iterator
    /// gives the processed event kind, and delta time in ticks
    Keyboard(EventKind, VarInt),
    /// change in tempo -> microseconds per quarter note
    Tempo(u32),
    /// this event marks the end of the current track
    EndOfTrack,
    /// possibly a valid MIDI event, but simply unsupported by this
    /// implementation
    Unsupported,
}

impl FromReader for MidiEvent
{
    fn read<R: Read + Seek>(reader: &mut R) -> std::io::Result<Self>
    {
        // delta time of the event, in ticks
        let dt = reader.decode::<VarInt>()?;
        // type of event, which is:
        //  if channel event: (event ty | channel)
        //  if meta event: (0xFF)
        //  if sys-ex event: (0xF*)
        let ty = reader.decode::<u8>()?;
        // match on the upper nibble of the `ty` byte, which:
        //  if channel event: the relevant type
        //  if meta event: just `0xF0`, double check that lower nibble = `0x0F`
        //  if sys-ex event: also `0xF0`, hence the double checking
        //
        // this discards the channel information of channel events, which this
        // implementation doesn't care about
        Ok(match ty & 0xF0
        {
            // note off: note number, velocity
            0x80 =>
            {
                let note = reader.decode::<u8>()?;
                let _vel = reader.decode::<u8>()?;

                // valid note on 88-key keyboard
                if let Some(note) = Note::new(note)
                {
                    MidiEvent::Keyboard(EventKind::Key(note, 0), dt)
                }
                // out of bounds
                else
                {
                    MidiEvent::Unsupported
                }
            },
            // note on: note number, velocity
            0x90 =>
            {
                let note = reader.decode::<u8>()?;
                let vel = reader.decode::<u8>()?;

                // valid note on 88-key keyboard
                if let Some(note) = Note::new(note)
                {
                    MidiEvent::Keyboard(EventKind::Key(note, vel.max(1)), dt)
                }
                // out of bounds
                else
                {
                    MidiEvent::Unsupported
                }
            },
            // controller value: num, val
            0xB0 =>
            {
                let num = reader.decode::<u8>()?;
                let val = reader.decode::<u8>()?;

                match num
                {
                    // sustain pedal
                    0x40 => MidiEvent::Keyboard(EventKind::Pedal(Pedal::Damper, val), dt),
                    // soft pedal
                    0x43 => MidiEvent::Keyboard(EventKind::Pedal(Pedal::Soft, val), dt),
                    // other...
                    _ => MidiEvent::Unsupported,
                }
            },
            // note aftertouch, pitch bend
            0xA0 | 0xE0 =>
            {
                // don't care about these events, but still need to read
                // them to completion
                let _arg1 = reader.decode::<u8>()?;
                let _arg2 = reader.decode::<u8>()?;

                MidiEvent::Unsupported
            },
            // program change, channel aftertouch
            0xC0 | 0xD0 =>
            {
                // don't care about these events, but still need to read
                // them to completion
                let _arg1 = reader.decode::<u8>()?;

                MidiEvent::Unsupported
            }
            // meta event | sys-ex event
            0xF0 =>
            {
                // meta event
                if ty == 0xFF
                {
                    // meta event type
                    let mty = reader.decode::<u8>()?;
                    // length of the event's data
                    let len = reader.decode::<VarInt>()?;

                    match mty
                    {
                        // end of track
                        0x2F =>
                        {
                            MidiEvent::EndOfTrack
                        },
                        0x51 =>
                        {
                            assert_eq!(*len, 3);

                            // 3 byte value
                            let raw = reader.decode::<[u8; 3]>()?;
                            // microseconds per quarter note
                            let mspqn = u32::from_be_bytes([0, raw[0], raw[1], raw[2]]);
                            // perform the conversion to BPM
                            MidiEvent::Tempo(mspqn)
                        },
                        // any other event, which we don't care about but still
                        // need to read to completion
                        _ =>
                        {
                            // consume data
                            reader.seek(std::io::SeekFrom::Current(*len as _))?;

                            MidiEvent::Unsupported
                        }
                    }
                }
                // sys-ex events
                else
                {
                    // length of the event's data
                    let len = reader.decode::<VarInt>()?;

                    // don't care about this event, but still need to consume its
                    // data
                    reader.seek(std::io::SeekFrom::Current(*len as _))?;

                    MidiEvent::Unsupported
                }
            },
            // ???
            _ => MidiEvent::Unsupported,
        })
    }
}

impl Event
{
    /// apply this state to the given keyboard and return the delta time of
    /// this event
    pub fn apply(self, keyboard: &mut Keyboard) -> std::time::Duration
    {
        match self.1
        {
            EventKind::Key(note, vel) => keyboard[note] = vel,
            EventKind::Pedal(pedal, val) => keyboard[pedal] = val,
        }
        self.0
    }
}