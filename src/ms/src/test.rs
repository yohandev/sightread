mod mid
{
    use crate::Keyboard;
    use crate::mid::*;

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
}

mod wav
{
    use std::time::Duration;

    use crate::wav::*;

    #[test]
    fn open_wav() -> crate::wav::Result<()>
    {
        let mut wav = WavFile::open("../../res/hungarian_rhapsody_no6.wav", 4096)?;

        let dur = Duration::from_secs_f32(1.0 / (wav.sample_rate() as f32 / wav.capacity() as f32));

        while let Some(pcm) = wav.next()?
        {
            std::thread::sleep(dur);

            print!("\x1B[2J\x1B[1;1H");
            println!("{}", format_pcm(pcm));
        }

        Ok(())
    }

    /// utility function to pretty print PCM data
    fn format_pcm(buf: &[f32]) -> String
    {
        let mut out = String::new();

        for smp in buf
        {
            // ▁▂▃▄▅▆▇█
            out.push(match smp.abs()
            {
                n if (0.125 * 0.0..0.125 * 1.0).contains(&n) => '▁',
                n if (0.125 * 1.0..0.125 * 2.0).contains(&n) => '▂',
                n if (0.125 * 2.0..0.125 * 3.0).contains(&n) => '▃',
                n if (0.125 * 3.0..0.125 * 4.0).contains(&n) => '▄',
                n if (0.125 * 4.0..0.125 * 5.0).contains(&n) => '▅',
                n if (0.125 * 5.0..0.125 * 6.0).contains(&n) => '▆',
                n if (0.125 * 6.0..0.125 * 7.0).contains(&n) => '▇',
                n if (0.125 * 7.0..0.125 * 8.0).contains(&n) => '█',
                _ => '█',
            });
        }
        out
    }
}