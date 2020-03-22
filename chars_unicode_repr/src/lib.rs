//! A unicode "thing" representation for cha(rs).
//!
//! The things that can be represented here are either codepoints (aka
//! the `char` type) or sequences of codepoints that are defined to
//! have a particular meaning - mostly emoji. The goal of this crate
//! is to serve as an intermediate and semantic representation of
//! unicode things that still serializes to a single `u64` (for
//! compatibility with the `fst` crate).

use std::fmt;

pub mod flag;
pub mod keycap;
pub mod variant;

use flag::Flag;
use keycap::Keycap;
use variant::VariantSelector;

/// A unit of information used for the organization, control, or
/// representation of textual data.
///
/// In this representation, abstract characters are cut into different
/// semantically meaningful variants (using a single codepoint, or as
/// varying combining character sequences), which should allow a
/// better display and explanation of the abstract character sequence
/// to the user of `cha(rs)`.
pub enum AbstractCharacter {
    /// An abstract character represented single codepoint (a Unicode
    /// scalar value, in reality).
    Codepoint(char),

    /// A "variation selector" sequence, with the "main" codepoint modified
    /// by a "variant selector" codepoint.
    ///
    /// There are various databases specifying the variations that are
    /// allowed here. These are:
    ///
    /// * [standardized variants](https://unicode.org/Public/UCD/latest/ucd/StandardizedVariants.txt)
    /// * [IVD Collections](http://www.unicode.org/Public/emoji/5.0/IVD_Collections.txt)
    /// * The `Basic_Emoji` group in [Emoji sequences](https://www.unicode.org/Public/emoji/13.0/emoji-sequences.txt)
    ///
    /// See also the [Variant sequences FAQ](http://unicode.org/faq/vs.html).
    Variation {
        main: char,
        variant: VariantSelector,
    },

    /// A "keycap" sequence in emoji sequences group
    /// `Emoji_Keycap_Sequence`.
    KeycapSequence(Keycap),

    /// A "flag" sequence, made up of two regional indicator symbols
    /// that form the two-letter ISO 3166 country code. Listed in the
    /// `RGI_Emoji_Flag_Sequence` group of the [Emoji sequences
    /// DB](https://www.unicode.org/Public/emoji/13.0/emoji-sequences.txt).
    ///
    /// See also [TR51's flag appendix](http://www.unicode.org/reports/tr51/tr51-16.html#Flags).
    FlagSequence(Flag),

    /// A sequence of code points joined by one or more zero-width
    /// joiner (ZWJ, `200D`) codepoints. These can be any arbitrary
    /// shape, so aren't structured in any way other than "utf-8
    /// encoded sequence of unicode code points".
    EmojiZWJSequence(String),
}

impl fmt::Display for AbstractCharacter {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            AbstractCharacter::Codepoint(c) => write!(f, "{}", c),
            AbstractCharacter::Variation { main, variant } => write!(f, "{}{}", main, variant),
            AbstractCharacter::KeycapSequence(k) => write!(f, "{}", k),
            AbstractCharacter::FlagSequence(fl) => write!(f, "{}", fl),
            AbstractCharacter::EmojiZWJSequence(s) => write!(f, "{}", s),
        }
    }
}
