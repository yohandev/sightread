use std::io::{ Read, Seek, Result, SeekFrom };

/// extension traits for types that are both `Read` and `Seek`,
/// namely `stdlib`'s `BufReader`
pub trait Stream: Read + Seek
{
    /// attempt to retrieve the given type from this stream
    #[inline]
    fn parse<T: FromStream>(&mut self) -> Result<T::Out> where Self: Sized
    {
        T::parse(self)
    }

    /// attenmpt to peek the given type from this stream without
    /// reading it
    fn peek<T: FromStream>(&mut self) -> Result<T::Out> where Self: Sized
    {
        let pos = self.stream_position()? as i64;
        let out = self.parse::<T>();
        let now = self.stream_position()? as i64;

        self.seek(SeekFrom::Current(pos - now))?;

        out
    }
}

/// types that can be constructed from a `Stream` of bytes
pub trait FromStream: Sized
{    
    // output read, usually `Self`
    type Out;

    /// attempt to construct `Self` from a stream of bytes
    fn parse(stream: &mut impl Stream) -> Result<Self::Out>;
}

// blanket implementation
impl<R : Read + Seek> Stream for R { }

/// macro to implement the `FromBytes` trait, which is basically
/// the same for all primitive types with the exception of the
/// concrete type, `$typ`
macro_rules! impl_from_bytes
{
    ($($typ:ty),*) =>
    {
        $(
        impl FromStream for $typ
        {
            type Out = Self;

            fn parse(stream: &mut impl Stream) -> Result<Self::Out>
            {
                // create byte buffer of appropriate size
                let mut buf = [0u8; std::mem::size_of::<Self>()];
                // read into buffer
                stream.read_exact(&mut buf)?;
                // create `Self` from buffer
                Ok(Self::from_be_bytes(buf))
            }
        }
        )*
    };
}
// implement for MIDI specific types
impl_from_bytes!(i32, i16, u8);

// typically used with `[u8; 4]` to compare against chunk tags
impl<const N: usize> FromStream for [u8; N]
{
    type Out = Self;

    fn parse(stream: &mut impl Stream) -> Result<Self::Out>
    {
        // create empty buffer
        let mut buf = [0u8; N];
        // fill buffer with relevant data
        stream.read_exact(&mut buf)?;
        // done
        Ok(buf)
    }
}

/// a MIDI variable length integer
pub struct VarInt;

impl FromStream for VarInt
{
    type Out = i32;

    fn parse(stream: &mut impl Stream) -> Result<Self::Out>
    {
        // accumulator for variable size integer
        let mut res = 0i32;

        loop
        {
            // read next byte
            let b = stream.parse::<u8>()?;
            // last bit set to one, continue reading
            if b & 0x80 != 0
            {
                res += (b & 0x7f) as i32;
                res <<= 7;
            }
            // last bytes
            else
            {
                break Ok(res + b as i32)
            }
        }
    }
}