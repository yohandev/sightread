use std::time::Duration;
use std::path::Path;

use self::evt::MidiEvent;

mod evt;
mod err;
mod msg;
mod io;

pub fn midi(path: impl AsRef<Path>)// -> impl Iterator<Item = (Duration, MidiEvent)>
{
    todo!()
}

