use crate::*;

#[test]
fn open_midi() -> Result<()>
{
    let mut keyboard = Keyboard::new();
    let mut midi = MidiFile::open("../../../res/MIDI-Unprocessed_25_R3_2011_MID--AUDIO_R3-D9_06_Track06_wav.midi")?;

    while let Some(event) = midi.next()?
    {
        std::thread::sleep(event.apply(&mut keyboard));

        // if let EventKind::Key(note, vel) = event.1
        // {
        //     println!("{}: {}", note, if vel > 0 { "on" } else { "off" });

        //     std::thread::sleep(event.0 * 10);
        // }
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", keyboard);
    }

    Ok(())
}