use std::{convert::{TryFrom, TryInto}, fmt::{ Debug, Display, Write }};
use std::str::FromStr;

/// represents a key on a traditional 88-key keyboard
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Key
{
    /// octave of the key, 1-7 + A0-B0, C8
    oct: Octave,
    /// A-G
    note: Note,
    /// natural, sharp, flat
    acc: Accent,
}

/// represents a musical note
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Note
{
    /// Do
    C,
    /// Re
    D,
    /// Mi
    E,
    /// Fa
    F,
    /// So
    G,
    /// La
    A,
    /// Ti
    B,
}

/// represents a music accidental sign
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Accent
{
    /// cancels out of a flat or a sharp(♮)
    Natural,
    /// play the note a semitone higher(♯)
    Sharp,
    /// play the note a semitone lower(♭)
    Flat,
}

/// represents a musical octave, 0-8
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Octave(pub u8);

impl Key
{
    /// create a new key, on an 88-key keyboard, from its components
    pub fn new(oct: Octave, note: Note, acc: Accent) -> Self
    {
        let key = Self { oct, note, acc };

        if Self::valid(oct, note, acc)
        {
            key
        }
        else
        {
            panic!("{:?} is out of bounds of an 88-key keyboard!", key)
        }
    }

    /// is the given key on an 88-key keyboard?
    pub fn valid(oct: Octave, note: Note, acc: Accent) -> bool
    {
        match oct.0
        {
            n if (1..=7).contains(&n) =>
            {
                true
            },
            0 =>
            {
                match note
                {
                    Note::B => true,
                    Note::A => !matches!(acc, Accent::Flat),
                    _ => false,
                }
            },
            8 =>
            {
                match note
                {
                    Note::C => !matches!(acc, Accent::Sharp),
                    _ => false,
                }
            },
            _ => false
        }
    }
}

impl Display for Note
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        Debug::fmt(&self, f)
    }
}

impl Display for Accent
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Accent::Natural => Ok(()),
            Accent::Sharp => f.write_char('♯'),
            Accent::Flat => f.write_char('♭'),
        }
    }
}

impl Display for Octave
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        Debug::fmt(&self.0, f)
    }
}

impl Display for Key
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        f.write_fmt(format_args!("{}{}{}", self.note, self.oct, self.acc))
    }
}

impl TryFrom<char> for Note
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

impl TryFrom<char> for Accent
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

impl TryFrom<char> for Octave
{
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error>
    {
        Ok(Self(value
            .to_digit(10)
            .map(|int| int as u8)
            .filter(|o| (0..=8).contains(o))
            .ok_or(())?
        ))
    }
}

impl FromStr for Key
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        // "C♭3", "C3♭", "C3b", or something along those lines
        let mut iter = s.chars();

        // character at position 0, note
        let note: Note = iter
            .next()
            .ok_or(())?
            .try_into()?;

        // character at position 1, accent or octave
        let one = iter
            .next()
            .ok_or(())?;
        // character at position 2, accent or octave or none
        let two = iter
            .next();
        // accent(#, b, natural) and octave(0..=8) order
        // is arbitrary
        let (acc, oct): (Option<Accent>, Option<Octave>) =
        (
            one.try_into().ok(),
            one.try_into().ok()
        );
        // resolve second item in (accent, octave) pair
        let (acc, oct): (Accent, Octave) = match (acc, oct)
        {
            // neither worked
            (None, None) => Err(()),
            // was accent, third is octave
            (Some(a), None) => Octave::try_from(two.ok_or(())?).map(|o| (a, o)),
            // was octave, then has accent?
            (None, Some(o)) => match two
            {
                // parse accent, sharp or flat
                Some(c) => Accent::try_from(c).map(|a| (a, o)),
                // no accent, natural
                None => Ok((Accent::Natural, o)),
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
        if Self::valid(oct, note, acc)
        {
            Ok(Self::new(oct, note, acc))
        }
        else
        {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::Key;

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
            let key = string.parse::<Key>();

            println!("\nParsed \"{}\", got: {:?}", string, key);
            if let Ok(key) = key
            {
                print!(" = {}", key);
            }
        }
    }
}