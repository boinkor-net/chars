//! A representation of the Unicode flag constituents.

use std::char::from_u32_unchecked;
use std::fmt;

/// Regional indicator symbols.
///
/// Unlike other representations, these symbols typically have no
/// meaning on their own and are used mainly in combination with
/// others.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RegionalIndicator {
    LetterA = 0x1F1E6,
    LetterB = 0x1F1E7,
    LetterC = 0x1F1E8,
    LetterD = 0x1F1E9,
    LetterE = 0x1F1EA,
    LetterF = 0x1F1EB,
    LetterG = 0x1F1EC,
    LetterH = 0x1F1ED,
    LetterI = 0x1F1EE,
    LetterJ = 0x1F1EF,
    LetterK = 0x1F1F0,
    LetterL = 0x1F1F1,
    LetterM = 0x1F1F2,
    LetterN = 0x1F1F3,
    LetterO = 0x1F1F4,
    LetterP = 0x1F1F5,
    LetterQ = 0x1F1F6,
    LetterR = 0x1F1F7,
    LetterS = 0x1F1F8,
    LetterT = 0x1F1F9,
    LetterU = 0x1F1FA,
    LetterV = 0x1F1FB,
    LetterW = 0x1F1FC,
    LetterX = 0x1F1FD,
    LetterY = 0x1F1FE,
    LetterZ = 0x1F1FF,
}

/// A "flag" sequence, made up of two regional indicator symbols
/// that form the two-letter ISO 3166 country code. Listed in the
/// `RGI_Emoji_Flag_Sequence` group of the [Emoji sequences
/// DB](https://www.unicode.org/Public/emoji/13.0/emoji-sequences.txt).
///
/// See also [TR51's flag appendix](http://www.unicode.org/reports/tr51/tr51-16.html#Flags).
pub struct Flag(RegionalIndicator, RegionalIndicator);

impl Into<char> for RegionalIndicator {
    fn into(self) -> char {
        unsafe { from_u32_unchecked(self as u32) }
    }
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let c1: char = self.0.into();
        let c2: char = self.1.into();
        write!(f, "{}{}", c1, c2)
    }
}
