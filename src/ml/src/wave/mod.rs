pub use error::{ Error, Result };

use crate::io::{ LittleEndian, Stream };

mod error;

#[cfg(test)]
mod test;

pub struct WavFile<S>
{
    /// number of channels in this file
    num_channels: u16,
    /// samples per second
    sample_rate: u32,
    /// bytes used by one sample over all the channels
    block_align: usize,
    /// function to read a single sample from one channel, using
    /// dynamic dispatch
    read_sample: fn(&mut S) -> Result<f32>,
    /// byte stream, to parse, from `.wav` file
    stream: S,
    /// bytes left to read in the stream
    left: usize,
    /// sample buffer, to pass off when iterating. contains PCM data
    /// read, with channels merged to one
    buf: Box<[f32]>,
}

impl<S: Stream<LittleEndian>> WavFile<S>
{
    /// `capacity`: samples to read at once
    pub fn new(mut stream: S, capacity: usize) -> Result<Self>
    {
        use Error::Parse;

        if stream.parse::<[u8; 4]>()? != *b"RIFF"               // file header - MThd
        {
            Err(Parse("Invalid file header(expected RIFF)"))?
        }
        let _len = stream.parse::<u32>()?;                      // file length

        if stream.parse::<[u8; 4]>()? != *b"WAVE"               // file format - WAVE
        {
            Err(Parse("Invalid file format(expected WAVE)"))?
        }
        if stream.parse::<[u8; 4]>()? != *b"fmt "               // format subchunk
        {
            Err(Parse("Invalid format subchunk(expected \"fmt \")"))?
        }
        if stream.parse::<u32>()? != 16                         // format subchunk size - 16
        {
            Err(Parse("Invalid fmt subchunk length(expected 16)"))?
        }
        if stream.parse::<u16>()? != 1                          // audio format - PCM
        {
            Err(Parse("File is not uncompressed PCM data!"))?
        }
        let num_channels = stream.parse::<u16>()?;              // number of channels
        let sample_rate = stream.parse::<u32>()?;               // sampling rate(hz)
        let _byte_rate = stream.parse::<u32>()?;                // sample_rate * num_channels * bits_per_sample / 8
        let block_align = stream.parse::<u16>()? as usize;      // num_channels * bits_per_sample / 8
                                                                // number of bytes for one sample with all channels
        let bits_per_sample = stream.parse::<u16>()?;           // bits per sample - 8, 16, 24, 32

        if stream.parse::<[u8; 4]>()? != *b"data"               // data subchunk
        {
            Err(Parse("Invalid data subchunk(expected \"data\")"))?
        }
        let left = stream.parse::<u32>()? as usize;             // bytes left to read in file

        let read_sample = match bits_per_sample                 // emulate dynamic dispatch
        {
            8 => todo!(),
            16 => Self::read_sample_16,
            24 => todo!(),
            32 => todo!(),
            _ => unimplemented!(),
        };

        Ok(Self
        {
            num_channels,
            sample_rate,
            block_align,
            read_sample,
            stream,
            left,
            buf: vec![0.0; capacity].into_boxed_slice()
        })
    }

    /// iterate over this `.wav` file's PCM data
    /// ```
    /// fn main() -> wave::Result<()>
    /// {
    ///     let mut wav = WavFile::open("fur_elise.wav")?;
    ///     
    ///     while let Some(pcm) = wav.next()?
    ///     {
    ///         // samples are merged to one channel
    ///         for sample in pcm { }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn next(&mut self) -> Result<Option<&[f32]>>
    {
        // no more PCM to read
        if self.left == 0 { return Ok(None) }

        for sample in &mut *self.buf
        {
            // read to end, return partial PCM buffer
            if self.left == 0
            {
                return Ok(Some(&self.buf))
            }

            *sample = 0.0;
            // sum all the channel's data...
            for _ in 0..self.num_channels
            {
                *sample += (self.read_sample)(&mut self.stream)?;
            }
            // ...then normalize back to -1.0..1.0 range
            *sample /= self.num_channels as f32;

            // increment bytes read
            self.left -= self.block_align;
        }
        // managed to fill buffer
        Ok(Some(&self.buf))
    }

    /// get this `.midi` file's sample rate(hz)
    pub fn sample_rate(&self) -> u32
    {
        self.sample_rate
    }

    /// max number of frames(all the samples per channel merged into one)
    /// returned by `WavFile::next`
    pub fn capacity(&self) -> usize
    {
        self.buf.len()
    }

    /// reads a single sample from a single channel, where bits
    /// per sample is 16
    fn read_sample_16(stream: &mut S) -> Result<f32>
    {
        /// convertion factor
        const CONVERT: f32 = -1.0 / (i16::MIN as f32);

        Ok(stream.parse::<i16>()? as f32 * CONVERT)
    }
}

impl WavFile<std::io::BufReader<std::fs::File>>
{
    /// open a MIDI file from its path
    pub fn open(path: impl AsRef<std::path::Path>, capacity: usize) -> Result<Self>
    {
        Self::new(std::io::BufReader::new(std::fs::File::open(path)?), capacity)
    }
}