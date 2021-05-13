/// represents the state for an 88-key keyboard, on/off
/// with velocity
#[derive(Debug, Clone, PartialEq)]
pub struct Keyboard
{
    /// keyboard keys, mapping A0♮..C8♮ indices to keys'
    /// respective "velocities" as passed in by MIDI files,
    /// a value of `0` indicating that key being released
    keys: [u8; 88],
    /// [soft pedal, damber pedal(sustain)] press amounts, as
    /// represented by MIDI files
    pedals: [u8; 2],
}

/// wrapper over a MIDI note code, permitting only those found on an
/// 88-key keyboard
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Note(u8);

/// an interval spanning the 12 notes on the chromatic scale
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Octave(u8);

/// white keys on a keyboard, denoting the C Major scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tone
{
    /// Do
    C = 0,
    /// Re
    D = 2,
    /// Mi
    E = 4,
    /// Fa
    F = 5,
    /// So
    G = 7,
    /// La
    A = 9,
    /// Ti
    B = 11,
}

/// synonymous to a music accidental, moving a half-step up or
/// down on the chromatic scale for `♯` and `♭` respectively.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Semitone
{
    /// `♮`
    ///
    /// indicates lack of a semitone, effectively cancelling out
    /// a sharp or flat acccidental
    Natural = 0,
    /// `♯`
    /// 
    /// indicates that the note is a semitone higher on the chromatic
    /// scale
    Sharp = 1,
    /// `♭`
    /// 
    /// indicates that the note is a semitone lower on the chromatic
    /// scale
    Flat = -1,
}

/// represents a normal piano's two pedals(mute omitted)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pedal
{
    /// the soft pedal
    Soft = 0,
    /// the damper pedal(sustain)
    Damper = 1,
}

impl Keyboard
{
    /// create a new `Keyboard` with zeroed out values
    pub fn new() -> Self
    {
        Self
        {
            keys: [0; 88],
            pedals: [0; 2],
        }
    }
}

impl Note
{
    /// lowest note on an 88-key keyboard(A0♮), and effectively the
    /// lowest permitted inner value for `Note`s
    pub const MIN: Self = Self(21);
    /// highest note on an 88-key keyboard(C8♮), and effectively the
    /// highest permitted inner value for `Note`s
    pub const MAX: Self = Self(108);

    /// construct a `Note` from its MIDI valeu 
    pub fn new(midi: u8) -> Option<Self>
    {
        if midi >= Self::MIN.0
        || midi <= Self::MAX.0
        {
            Some(Self(midi))
        }
        else
        {
            None
        }
    }

    /// construct a `Note`, but from its musical notation rather than
    /// MIDI note value
    pub fn new2(oct: Octave, tone: Tone, semi: Semitone) -> Option<Self>
    {
        // make this a midi note number
        let note = ((oct.0 as isize + 2) * 12)  // octave
            + (tone as isize)                   // tone
            + (semi as isize);                  // semitone
        
        Self::new(note as u8)
    }

    /// get the octave of this `Note`
    ///
    /// for `Note::octave` and `Note::tone`, ambiguous cases
    /// are resolved by favouring sharps over flats
    /// (ie. `C♯` vs `D♭` yields `C♯`) 
    pub fn octave(&self) -> Octave
    {
        Octave((self.0 / 12) - 2)
    }

    /// get the tone and semitone of this `Note`
    ///
    /// for `Note::octave` and `Note::tone`, ambiguous cases
    /// are resolved by favouring sharps over flats
    /// (ie. `C♯` vs `D♭` yields `C♯`) 
    pub fn tone(&self) -> (Tone, Semitone)
    {
        use Semitone::*;
        use Tone::*;

        match self.0 % 12
        {
            0 => (C, Natural),
            1 => (C, Sharp),
            2 => (D, Natural),
            3 => (D, Sharp),
            4 => (E, Natural),
            5 => (F, Natural),
            6 => (F, Sharp),
            7 => (G, Natural),
            8 => (G, Sharp),
            9 => (A, Natural),
            10 => (A, Sharp),
            11 => (B, Natural),
            _ => unreachable!()
        }
    }
}

impl std::ops::Index<Note> for Keyboard
{
    type Output = u8;

    fn index(&self, index: Note) -> &Self::Output
    {
        &self.keys[(index.0 - Note::MIN.0) as usize]
    }
}

impl std::ops::IndexMut<Note> for Keyboard
{
    fn index_mut(&mut self, index: Note) -> &mut Self::Output
    {
        &mut self.keys[(index.0 - Note::MIN.0) as usize]
    }
}

impl std::ops::Index<Pedal> for Keyboard
{
    type Output = u8;

    fn index(&self, index: Pedal) -> &Self::Output
    {
        &self.pedals[index as usize]
    }
}

impl std::ops::IndexMut<Pedal> for Keyboard
{
    fn index_mut(&mut self, index: Pedal) -> &mut Self::Output
    {
        &mut self.pedals[index as usize]
    }
}

impl std::fmt::Display for Tone
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for Semitone
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Self::Natural => Ok(()),
            Self::Sharp => write!(f, "{}", '♯'),
            Self::Flat => write!(f, "{}", '♭'),
        }
    }
}

impl std::fmt::Display for Octave
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{:?}", self.0)
    }
}

impl std::fmt::Display for Note
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let octv = self.octave();
        let (tone, semi) = self.tone();

        f.write_fmt(format_args!("{}{}{}", tone, octv, semi))
    }
}

impl std::convert::TryFrom<char> for Tone
{
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error>
    {
        match value
        {
            'C' => Ok(Self::C),
            'D' => Ok(Self::D),
            'E' => Ok(Self::E),
            'F' => Ok(Self::F),
            'G' => Ok(Self::G),
            'A' => Ok(Self::A),
            'B' => Ok(Self::B),
            _ => Err(())
        }
    }
}

impl std::convert::TryFrom<char> for Semitone
{
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error>
    {
        match value
        {
            '♯' | '#' => Ok(Self::Sharp),
            '♭' | 'b' => Ok(Self::Flat),
            _ => Err(())
        }
    }
}

impl std::convert::TryFrom<char> for Octave
{
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error>
    {
        Ok(Self(value.to_digit(10).ok_or(())? as u8))
    }
}

impl std::str::FromStr for Note
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        use std::convert::{ TryFrom, TryInto };

        // "C♭3", "C3♭", "C3b", or something along those lines
        let mut iter = s.chars();

        // character at position 0, tone
        let tone: Tone = iter
            .next()
            .ok_or(())?
            .try_into()?;

        // character at position 1, semitone or octave
        let one = iter
            .next()
            .ok_or(())?;
        // character at position 2, semitone or octave or none
        let two = iter
            .next();
        // semitone(#, b, natural) and octave order
        // is arbitrary
        let (semi, octv): (Option<Semitone>, Option<Octave>) =
        (
            one.try_into().ok(),
            one.try_into().ok()
        );
        // resolve second item in (semitone, octave) pair
        let (semi, octv): (Semitone, Octave) = match (semi, octv)
        {
            // neither worked
            (None, None) => Err(()),
            // was semitone, third is octave
            (Some(a), None) => Octave::try_from(two.ok_or(())?).map(|o| (a, o)),
            // was octave, then has semitone?
            (None, Some(o)) => match two
            {
                // parse semitone, sharp or flat
                Some(c) => Semitone::try_from(c).map(|a| (a, o)),
                // no semitone, natural
                None => Ok((Semitone::Natural, o)),
            },
            // bruh
            (Some(_), Some(_)) => unreachable!("how??"),
        }?;
        // you're still going?!
        if iter.next().is_some()
        {
            return Err(())
        }

        // valid note on 88-key keyboard?
        Self::new2(octv, tone, semi).ok_or(())
    }
}

#[cfg(test)]
mod tests
{
    use super::Note;

    #[test]
    fn parse()
    {
        let input = [
            "C♭3", // pass
            "D♭5", // pass
            "E♯1", // pass
            "C3♭", // pass
            "D5♭", // pass
            "E1♯", // pass
            "A0",  // pass
            "C",   // fail
            "C4",  // pass
            "D9",  // fail
        ];

        for string in &input
        {
            let key = string.parse::<Note>();

            println!("\nParsed \"{}\", got: {:?}", string, key);
            if let Ok(key) = key
            {
                print!(" = {}", key);
            }
        }
    }
}