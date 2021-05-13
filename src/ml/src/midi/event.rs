use super::util::*;

/// a MIDI [EventKind] + its delta time compared to the last event.
/// a delta time of 0 indicates this event and the last are played simultaenously
///
/// [EventKind]: EventKind
pub struct Event
{
    /// the inner MIDI event
    pub kind: EventKind,
    /// delta time of the event, in MIDI ticks
    pub dt: VarInt,
}

/// MIDI events kind
///
/// only includes events this specific implementation cares about, ie. anything `.midi` files
/// within the `maestro` dataset might contain, and nothing more
pub enum EventKind
{
    /// a note was toggled off
    KeyReleased { note: u8 },
    /// a note was toggled on
    KeyPressed { note: u8, vel: u8 },
    /// change in pressure applied to the damper pedal(sustain)
    DamperPedal { val: u8 },
    /// change in pressure applied to the soft pedal
    SoftPedal { val: u8 },
    /// indicates a change in tempo
    Tempo { bpm: f32 },
    /// signifies the end of the current track
    EndOfTrack,
    /// a (maybe) valid MIDI event that the parser simply isn't
    /// designed to handle; its data is dropped
    Unsupported,
}

impl FromReader for Event
{
    fn read<R: std::io::Read + std::io::Seek>(reader: &mut R) -> std::io::Result<Self>
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
        let kind = match ty & 0xF0
        {
            // note off: note number, velocity
            0x80 =>
            {
                let note = reader.decode::<u8>()?;
                let _vel = reader.decode::<u8>()?;

                EventKind::KeyReleased { note }
            },
            // note on: note number, velocity
            0x90 =>
            {
                let note = reader.decode::<u8>()?;
                let vel = reader.decode::<u8>()?;

                EventKind::KeyPressed { note, vel }
            },
            // controller value: num, val
            0xB0 =>
            {
                let num = reader.decode::<u8>()?;
                let val = reader.decode::<u8>()?;

                match num
                {
                    // sustain pedal
                    0x40 => EventKind::DamperPedal { val },
                    // soft pedal
                    0x43 => EventKind::SoftPedal { val },
                    // other...
                    _ => EventKind::Unsupported,
                }
            },
            // note aftertouch, program change, channel aftertouch,
            // pitch bend
            0xA0 | 0xC0 | 0xD0 | 0xE0 =>
            {
                // don't care about these events, but still need to read
                // them to completion
                let _arg1 = reader.decode::<u8>()?;
                let _arg2 = reader.decode::<u8>()?;

                EventKind::Unsupported
            },
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
                        0x2F => EventKind::EndOfTrack,
                        0x51 =>
                        {
                            assert_eq!(*len, 3);

                            // 3 byte value
                            let raw = reader.decode::<[u8; 3]>()?;
                            // microseconds per quarter note
                            let mspqn = u32::from_be_bytes([0, raw[0], raw[1], raw[2]]);
                            // perform the conversion to BPM
                            EventKind::Tempo { bpm: 60000000.0 / mspqn as f32 }
                        },
                        // any other event, which we don't care about but still
                        // need to read to completion
                        _ =>
                        {
                            // consume data
                            reader.seek(std::io::SeekFrom::Current(*len as _))?;

                            EventKind::Unsupported
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

                    EventKind::Unsupported
                }
            },
            // ???
            _ => EventKind::Unsupported,
        };

        Ok(Self { kind, dt })
    }
}