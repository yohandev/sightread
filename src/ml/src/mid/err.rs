/// error thrown while reading a [MidiFile]
///
/// [MidiFile]: MidiFile
#[derive(Debug)]
pub enum Error
{
    /// some sort of file in/out error
    Io(std::io::Error),

    /// expected "MThd" as the first file header
    InvalidFileHeader,
    /// expected header "MThd" to be of length 6
    InvalidHeaderLength,
    /// only supports single-track(type = 0) MIDI files
    InvalidTracksAmount,
    /// ticks per quater note use unsupported SMPTE timecode
    InvalidTimingMode,
    /// expected "MTrk" as the header for a track
    InvalidTrackheader,
}

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
            Error::InvalidFileHeader => f.write_str("expected \"MThd\""),
            Error::InvalidHeaderLength => f.write_str("expected length of 6"),
            Error::InvalidTracksAmount => f.write_str("more than 1 track not supported"),
            Error::InvalidTimingMode => f.write_str("SMPTE timecode not supported"),
            Error::InvalidTrackheader => f.write_str("expected \"Mtrk\""),
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