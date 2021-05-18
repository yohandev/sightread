use std::time::Duration;
 
use crate::key::{ Keyboard, Note, Pedal };

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