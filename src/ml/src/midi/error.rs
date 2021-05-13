/// error thrown while reading a [Midi] file
///
/// [Midi]: super::Midi
#[derive(Debug)]
pub enum Error
{
    /// some sort of file in/out error
    Io(std::io::Error),

    /// expected "MThd" as the first file header
    /// or its length wasn't 6
    InvalidHeaderChunk,
    /// ticks per quater note use unsupported SMPTE timecode
    UnsupportedTimingMode,
    /// expected "MTrk" as the header for a track
    InvalidTrackChunk,
}

/// a MIDI result
pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
    {
        match self
        {
            Error::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Error::Io(err) => err.fmt(f),
            Error::InvalidHeaderChunk => f.write_str("chunk \"MThd\" failed to parse"),
            Error::UnsupportedTimingMode => f.write_str("SMPTE timecode not supported"),
            Error::InvalidTrackChunk => f.write_str("chunk \"Mtrk\" failed to parse"),
        }
    }
}

impl From<std::io::Error> for Error
{
    fn from(err: std::io::Error) -> Self
    {
        Self::Io(err)
    }
}