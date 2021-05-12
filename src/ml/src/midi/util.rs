use std::{io::{ Read, Seek }, ops::Deref};

/// trait for types that can be constructed from a stream of
/// bytes for a [Read] object
///
/// [Read]: std::io::Read
pub trait FromReader: Sized
{
    /// attempt to read a `Self` from the reader
    fn read<R: Read + Seek>(reader: &mut R) -> std::io::Result<Self>;
}

/// extension trait to make [FromReader] syntax cleaner
///
/// [FromReader]: FromReader
pub trait ReaderExt: Read + Seek + Sized
{
    /// read the given data type `T` from `self`
    fn decode<T: FromReader>(&mut self) -> std::io::Result<T>
    {
        T::read(self)
    }
}

/// a MIDI variable-length integer
///
/// given a big-endian array of bytes, it's represented by groups
/// of 7-bits with the top bit set to signify that another byte
/// follows
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VarInt(u32);

/// macro to implement the `FromBytes` trait, which is basically
/// the same for all primitive types with the exception of the
/// concrete type, `$typ`
macro_rules! impl_from_bytes
{
    ($($typ:ty),*) =>
    {
        $(
        impl FromReader for $typ
        {
            fn read<R: Read + Seek>(reader: &mut R) -> std::io::Result<Self>
            {
                // create byte buffer of appropriate size
                let mut buf = [0u8; std::mem::size_of::<Self>()];
                // read into buffer
                reader.read_exact(&mut buf)?;
                // create `Self` from buffer
                Ok(Self::from_be_bytes(buf))
            }
        }
        )*
    };
}

// implement for MIDI specific types
impl_from_bytes!(u32, u16, u8, i8);

// typically used with `[u8; 4]` to compare against chunk tags
impl<const N: usize> FromReader for [u8; N]
{
    fn read<R: Read + Seek>(reader: &mut R) -> std::io::Result<Self>
    {
        // create empty buffer
        let mut buf = [0u8; N];
        // fill buffer with relevant data
        reader.read_exact(&mut buf)?;
        // done
        Ok(buf)
    }
}

// implementation for VarInt is a bit special
impl FromReader for VarInt
{
    fn read<R: Read + Seek>(bytes: &mut R) -> std::io::Result<Self>
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

// blanked implementation
impl<R: Read + Seek + Sized> ReaderExt for R { }

impl Deref for VarInt
{
    type Target = u32;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}