//! A representation of the unicode keycap sequences.

use crate::VariantSelector;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::char::from_u32_unchecked;
use std::{convert::TryFrom, fmt, str::FromStr};

/// Possible keycap variations.
///
/// These include:
/// * Digits [`0`][Keycap::Digit0] through [`9`][Keycap::Digit9]: 0️⃣, 1️⃣, 2️⃣, 3️⃣, 4️⃣, 5️⃣, 6️⃣, 7️⃣, 8️⃣, 9️⃣
/// * The [Star/`*`][Keycap::Star] keycap: #️⃣
/// * The [Pound/Octophorpe/Sharp/`#`][`Keycap::Pound`] keycap: *️⃣
#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
pub enum Keycap {
    Digit0 = Keycap::ZERO,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,

    Star = Keycap::STAR,

    Pound = Keycap::POUND,
}

impl Keycap {
    const COMBINING_ENCLOSING_KEYCAP: char = '\u{20E3}';

    const ZERO: isize = b'0' as isize;

    const STAR: isize = b'*' as isize;

    const POUND: isize = b'#' as isize;
}

impl fmt::Display for Keycap {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}{}{}",
            unsafe { from_u32_unchecked(*self as u32) },
            VariantSelector::Emoji,
            Keycap::COMBINING_ENCLOSING_KEYCAP
        )
    }
}

impl TryFrom<char> for Keycap {
    /// The original char that is not a keycap:
    type Error = char;

    fn try_from(f: char) -> Result<Self, Self::Error> {
        Keycap::from_u32(f as u32).ok_or(f)
    }
}

impl FromStr for Keycap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chs = s.chars();
        let cap = Keycap::try_from(chs.next().ok_or(())?).map_err(|_| ())?;
        match (chs.next(), chs.next(), chs.next()) {
            (
                Some(VariantSelector::EMOJI_SELECTOR),
                Some(Keycap::COMBINING_ENCLOSING_KEYCAP),
                None,
            ) => Ok(cap),
            (_, _, _) => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn keycap_from_str() {
        assert_eq!(Ok(Keycap::Digit1), Keycap::from_str("1️⃣"));
        assert_eq!(Ok(Keycap::Digit9), Keycap::from_str("9️⃣"));
        assert_eq!(Ok(Keycap::Star), Keycap::from_str("*️⃣"));
        assert_eq!(Ok(Keycap::Pound), Keycap::from_str("#️⃣"));

        assert_eq!(Err(()), Keycap::from_str("#️⃣b"));
        assert_eq!(Err(()), Keycap::from_str("b️⃣"));
        assert_eq!(Err(()), Keycap::from_str("b⃣"));
    }
}
