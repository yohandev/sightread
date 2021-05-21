use super::*;

#[test]
fn open_midi() -> Result<()>
{
    let mut keyboard = Keyboard::new();
    let mut midi = MidiFile::open("../../res/hungarian_rhapsody_no6.midi")?;

    while let Some(event) = midi.next()?
    {
        std::thread::sleep(event.apply(&mut keyboard));

        print!("\x1B[2J\x1B[1;1H");
        println!("{}", keyboard);
    }

    Ok(())
}