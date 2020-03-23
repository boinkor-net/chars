use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::char::from_u32_unchecked;
use std::convert::TryFrom;
use std::fmt;

/// The "Fitzpatrick" or skin-tone modifier for Emoji sequences of
/// type `RGI_Emoji_Modifier_Sequence`.
#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
pub enum EmojiModifier {
    Type12 = 0x1F3FB,
    Type3,
    Type4,
    Type5,
    Type6,
}

impl Into<char> for &EmojiModifier {
    fn into(self) -> char {
        unsafe { from_u32_unchecked(*self as u32) }
    }
}

impl TryFrom<char> for EmojiModifier {
    /// The original char that is not a mongolian variant selector
    type Error = char;

    fn try_from(f: char) -> Result<Self, Self::Error> {
        EmojiModifier::from_u32(f as u32).ok_or(f)
    }
}

impl fmt::Display for EmojiModifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let c: char = self.into();
        write!(f, "{}", c)
    }
}
