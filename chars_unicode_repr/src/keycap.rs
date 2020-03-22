//! A representation of the unicode keycap sequences.

use crate::VariantSelector;
use std::char::from_u32_unchecked;
use std::fmt;

/// Possible keycap variations.
///
/// These include:
/// * Digits [`0`][Keycap::Digit0] through [`9`][Keycap::Digit9]: 0️⃣, 1️⃣, 2️⃣, 3️⃣, 4️⃣, 5️⃣, 6️⃣, 7️⃣, 8️⃣, 9️⃣
/// * The [Star/`*`][Keycap::Star] keycap: #️⃣
/// * The [Pound/Octophorpe/Sharp/`#`][`Keycap::Pound`] keycap: *️⃣
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Keycap {
    Digit0 = (b'0' + 0) as isize,
    Digit1 = (b'0' + 1) as isize,
    Digit2 = (b'0' + 2) as isize,
    Digit3 = (b'0' + 3) as isize,
    Digit4 = (b'0' + 4) as isize,
    Digit5 = (b'0' + 5) as isize,
    Digit6 = (b'0' + 6) as isize,
    Digit7 = (b'0' + 7) as isize,
    Digit8 = (b'0' + 8) as isize,
    Digit9 = (b'0' + 9) as isize,

    Star = b'*' as isize,

    Pound = b'#' as isize,
}

const COMBINING_ENCLOSING_KEYCAP: char = '\u{20E3}';

impl fmt::Display for Keycap {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}{}{}",
            unsafe { from_u32_unchecked(*self as u32) },
            VariantSelector::Emoji,
            COMBINING_ENCLOSING_KEYCAP
        )
    }
}
