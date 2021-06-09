/// error thrown while reading a [WavFile]
///
/// [Wav]: super::WavFile
#[derive(Debug)]
pub enum Error
{
    /// some sort of file in/out error
    Io(std::io::Error),
    /// some sort of `.wav` parse error
    Parse(&'static str),
}

/// a `.wav` result
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
            Error::Parse(err) => write!(f, ".wav Parse Error: {}", err)
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