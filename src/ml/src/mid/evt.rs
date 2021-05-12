use crate::key::{ Key, Velocity };

/// all types of supported MIDI events, ie. those relevant to an acoustic piano
pub enum MidiEvent
{
    /// key is pressed, equivalnet to MIDI `NoteOn` event
    KeyPressed(Key, Velocity),
    /// key is released, equivalent to MIDI `NoteOff` event
    KeyReleased(Key),
    /// sustain pedal press amount changed
    Sustain(/* TODO */),
    /// soft pedal press amount changed
    SoftPedal(/* TODO */)
}
