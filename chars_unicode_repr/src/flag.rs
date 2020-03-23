//! A representation of the Unicode flag constituents.

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::char::from_u32_unchecked;
use std::{convert::TryFrom, fmt, str::FromStr};

/// Regional indicator symbols.
///
/// Unlike other representations, these symbols typically have no
/// meaning on their own and are used mainly in combination with
/// others.
#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
pub enum RegionalIndicator {
    LetterA = 0x1F1E6,
    LetterB,
    LetterC,
    LetterD,
    LetterE,
    LetterF,
    LetterG,
    LetterH,
    LetterI,
    LetterJ,
    LetterK,
    LetterL,
    LetterM,
    LetterN,
    LetterO,
    LetterP,
    LetterQ,
    LetterR,
    LetterS,
    LetterT,
    LetterU,
    LetterV,
    LetterW,
    LetterX,
    LetterY,
    LetterZ,
}

impl Into<char> for RegionalIndicator {
    fn into(self) -> char {
        unsafe { from_u32_unchecked(self as u32) }
    }
}

impl TryFrom<char> for RegionalIndicator {
    /// The original char that is not a keycap:
    type Error = char;

    fn try_from(f: char) -> Result<Self, Self::Error> {
        RegionalIndicator::from_u32(f as u32).ok_or(f)
    }
}

/// A "flag" sequence, made up of two regional indicator symbols
/// that form the two-letter ISO 3166 country code. Listed in the
/// `RGI_Emoji_Flag_Sequence` group of the [Emoji sequences
/// DB](https://www.unicode.org/Public/emoji/13.0/emoji-sequences.txt).
///
/// See also [TR51's flag appendix](http://www.unicode.org/reports/tr51/tr51-16.html#Flags).
#[derive(Debug, Clone, PartialEq)]
pub struct Flag(RegionalIndicator, RegionalIndicator);

impl Flag {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn new(f1: RegionalIndicator, f2: RegionalIndicator) -> Flag {
        Flag(f1, f2)
    }
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let c1: char = self.0.into();
        let c2: char = self.1.into();
        write!(f, "{}{}", c1, c2)
    }
}

impl FromStr for Flag {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chs = s.chars();
        let l1 = RegionalIndicator::try_from(chs.next().ok_or(())?).map_err(|_| ())?;
        let l2 = RegionalIndicator::try_from(chs.next().ok_or(())?).map_err(|_| ())?;
        if chs.next().is_none() {
            Ok(Flag(l1, l2))
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bounds() {
        assert_eq!(RegionalIndicator::LetterA as isize, 0x1F1E6 as isize);
        assert_eq!(RegionalIndicator::LetterZ as isize, 0x1F1FF as isize);
    }

    #[test]
    fn flag_from_str() {
        assert_eq!(
            Ok(Flag(RegionalIndicator::LetterA, RegionalIndicator::LetterT)),
            Flag::from_str("🇦🇹")
        );
        assert_eq!(Err(()), Flag::from_str("🇦🇹b"));
        assert_eq!(Err(()), Flag::from_str("🇦t"));
    }
}
