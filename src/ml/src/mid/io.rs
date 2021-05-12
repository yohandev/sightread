use std::io::{ Read, BufReader };
use std::path::Path;
use std::fs::File;

/// wrapper around a file reader
///
/// utility for parsing big-endian `.midi` files
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stream<R>
{
    /// input data reader
    read: R,
}

/// trait for numerical types that can be created from their
/// big endian byte stream representation
pub trait FromBytes: Sized
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
pub struct VarInt(u32);

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