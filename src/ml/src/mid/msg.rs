use std::time::Duration;

use crate::key::Key;

/// a MIDI [EventKind] + its delta time compared to the last event.
/// a delta time of 0 indicates this event and the last are played simultaenously
///
/// [EventKind]: EventKind
pub struct Event
{
    /// the inner MIDI event
    kind: EventKind,
    /// delta time of the event, tempo adjusted and in real-life durations
    dt: Duration,
}

/// MIDI events kind
///
/// only includes events this specific implementation cares about, ie. anything `.midi` files
/// within the `maestro` dataset might contain, and nothing more
pub enum EventKind
{
    /// a note was toggled off
    KeyReleased { key: Key },
    /// a note was toggled on
    KeyPressed { key: Key, vel: u8 },
    /// change in pressure applied to the damper pedal(sustain)
    DamperPedal { val: u8 },
    /// change in pressure applied to the soft pedal
    SoftPedal { val: u8 }
}

