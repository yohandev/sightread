use std::io::{ BufReader, Read };
use std::error::Error;
use std::fmt::Display;
use std::path::Path;
use std::fs::File;

/// a `.midi` file parser and iterator
pub struct MidiFile
{
    /// array of tracks, where each track is the encoded
    /// MIDI data for that track(to be decoded lazily when
    /// iterating events)
    tracks: Box<[Box<[u8]>]>
}

/// error thrown while reading a [MidiFile]
///
/// [MidiFile]: MidiFile
#[derive(Debug)]
pub enum MidiError
{
    /// some sort of file in/out error
    Io(std::io::Error),

    /// expected "MThd" as the first file header
    InvalidFileHeader,
    /// expected header "MThd" to be of length 6
    InvalidHeaderLength,
    /// ticks per quater note use unsupported SMPTE timecode
    InvalidTimingMode,
    /// expected "MTrk" as the header for a track
    InvalidTrackheader,
}

/// wrapper around a file reader
///
/// utility for parsing big-endian `.midi` files
#[derive(Debug, Clone, PartialEq, Eq)]
struct Stream<R>
{
    /// input data reader
    read: R,
}

/// trait for numerical types that can be created from their
/// big endian byte stream representation
trait FromBytes: Sized
{
    /// create `Self` from its representation of as a byte array
    /// in big endian
    fn read<R: Read>(bytes: &mut R) -> std::io::Result<Self>;
}

/// a MIDI variable-length integer
///
/// given a big-endian array of bytes, it's represented by groups
/// of 7-bits with the top bit set to signify that another byte
/// follows
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct VarInt(u32);

impl MidiFile
{
    /// opens and parses a MIDI file
    pub fn open(path: impl AsRef<Path>) -> Result<(), MidiError>
    {
        let mut stream = Stream::open(path)?;

        // read header
        if &stream.readn::<u8, 4>()? != b"MThd"
        {
            return Err(MidiError::InvalidFileHeader)
        }
        // read header length
        if stream.read::<u32>()? != 6
        {
            return Err(MidiError::InvalidHeaderLength)
        }
        // read header data
        let _format = stream.read::<u16>()?;
        // num tracks iterator
        let num_tracks = 0..stream.read::<u16>()?;
        // timecode
        let timing = stream.read::<u16>()?;
        if timing & 0x8000 != 0
        {
            return Err(MidiError::InvalidTimingMode)
        }
        // initialize tracks
        let tracks = num_tracks.map(|_|
        {
            // track chunk header
            if &stream.readn::<u8, 4>()? != b"MTrk"
            {
                return Err(MidiError::InvalidTrackheader)
            }
            // track length
            let len = stream.read::<u32>()? as usize;
            // buffer of bytes containing track data/events
            let mut buf = vec![0u8; len].into_boxed_slice();

            // read relevant data into buffer
            stream.read_into(&mut buf)?;

            Ok(buf)
        })
        .collect::<Result<Box<[_]>, _>>()?;

        

        Ok(())
    }
}

impl Error for MidiError
{
    fn source(&self) -> Option<&(dyn Error + 'static)>
    {
        match self
        {
            MidiError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl Display for MidiError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            MidiError::Io(err) => err.fmt(f),
            MidiError::InvalidFileHeader => f.write_str("expected \"MThd\""),
            MidiError::InvalidHeaderLength => f.write_str("expected length of 6"),
            MidiError::InvalidTimingMode => f.write_str("SMPTE timecode not supported"),
            MidiError::InvalidTrackheader => f.write_str("expected \"Mtrk\""),
        }
    }
}

impl From<std::io::Error> for MidiError
{
    fn from(err: std::io::Error) -> Self
    {
        Self::Io(err)
    }
}

impl Stream<BufReader<File>>
{
    /// opens a new stream from a file, given its path
    pub fn open(path: impl AsRef<Path>) -> std::io::Result<Self>
    {
        Ok(Self { read: BufReader::new(File::open(path)?) })
    }
}

impl<R: Read> Stream<R>
{
    /// read the given numerical type from the file stream
    pub fn read<T: FromBytes>(&mut self) -> std::io::Result<T>
    {
        T::read(&mut self.read)
    }

    /// read an array of the given numerical type from the file stream
    pub fn readn<T: FromBytes, const N: usize>(&mut self) -> std::io::Result<[T; N]> where T: Copy + Default
    {
        // output array
        let mut out = [T::default(); N];

        // populate array from stream
        for ele in &mut out
        {
            *ele = T::read(&mut self.read)?;
        }
        Ok(out)
    }

    /// read the exact amount needed to fill `buf`
    pub fn read_into(&mut self, buf: &mut [u8]) -> std::io::Result<()>
    {
        self.read.read_exact(buf)
    }
}

/// macro to implement the `FromBytes` trait, which is basically
/// the same for all primitive types with the exception of the
/// concrete type, `$typ`
macro_rules! impl_from_bytes
{
    ($($typ:ty),*) =>
    {
        $(
        impl FromBytes for $typ
        {
            fn read<R: Read>(bytes: &mut R) -> std::io::Result<Self>
            {
                // create byte buffer of appropriate size
                let mut buf = [0u8; std::mem::size_of::<Self>()];
                // read into buffer
                bytes.read_exact(&mut buf)?;
                // create `Self` from buffer
                Ok(Self::from_be_bytes(buf))
            }
        }
        )*
    };
}
// implement for MIDI specific types
impl_from_bytes!(u32, u16, u8, i8);
// implementation for VarInt is a bit special
impl FromBytes for VarInt
{
    fn read<R: Read>(bytes: &mut R) -> std::io::Result<Self>
    {
        // accumulator for variable size integer
        let mut res = 0u32;

        loop
        {
            // read next byte
            let b = u8::read(bytes)?;
            // last bit set to one, continue reading
            if b & 0x80 != 0
            {
                res += (b & 0x7f) as u32;
                res <<= 7;
            }
            // last bytes
            else
            {
                break Ok(Self(res + b as u32))
            }
        }
    }
}