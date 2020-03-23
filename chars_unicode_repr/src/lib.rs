//! A unicode "thing" representation for cha(rs).
//!
//! The things that can be represented here are either codepoints (aka
//! the `char` type) or sequences of codepoints that are defined to
//! have a particular meaning - mostly emoji. The goal of this crate
//! is to serve as an intermediate and semantic representation of
//! unicode things that still serializes to a single `u64` (for
//! compatibility with the `fst` crate).

use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

pub mod emoji_modifier;
pub mod flag;
pub mod keycap;
pub mod variant;

use emoji_modifier::EmojiModifier;
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
#[derive(Debug, PartialEq, Clone)]
pub enum AbstractCharacter {
    /// An abstract character represented as a single codepoint (a
    /// Unicode scalar value, in reality).
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
    Variation(char, VariantSelector),

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

    /// An emoji sequence modified with a (typically) skin-tone.
    ///
    /// See also [TR51's 1.4.6 on emoji
    /// sets](http://www.unicode.org/reports/tr51/tr51-16.html#def_std_emoji_modifier_sequence_set).
    EmojiModifierSequence(char, EmojiModifier),

    // TODO: Tag sequences (like they do with the various british flags for some reason?!)
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
            AbstractCharacter::Variation(main, variant) => write!(f, "{}{}", main, variant),
            AbstractCharacter::KeycapSequence(k) => write!(f, "{}", k),
            AbstractCharacter::FlagSequence(fl) => write!(f, "{}", fl),
            AbstractCharacter::EmojiModifierSequence(c, m) => write!(f, "{}{}", c, m),
            AbstractCharacter::EmojiZWJSequence(s) => write!(f, "{}", s),
        }
    }
}

impl From<&str> for AbstractCharacter {
    fn from(s: &str) -> Self {
        match (Flag::from_str(s), Keycap::from_str(s)) {
            (Ok(f), Err(_)) => return AbstractCharacter::FlagSequence(f),
            (Err(_), Ok(k)) => return AbstractCharacter::KeycapSequence(k),
            (_, _) => {}
        };
        let mut chs = s.chars();
        match (chs.next(), chs.next(), chs.next()) {
            (Some(c), None, None) => return AbstractCharacter::Codepoint(c),
            (Some(c1), Some(c2), None) => {
                if let Ok(var) = VariantSelector::try_from(c2) {
                    return AbstractCharacter::Variation(c1, var);
                }
                if let Ok(modifier) = EmojiModifier::try_from(c2) {
                    return AbstractCharacter::EmojiModifierSequence(c1, modifier);
                }
            }
            _ => {}
        };
        //  TODO: is this right? We should probably only go for sequences with emoji and ZWJ in them.
        AbstractCharacter::EmojiZWJSequence(s.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::flag::{Flag, RegionalIndicator};
    use super::keycap::Keycap;
    use super::variant::VariantSelector;
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!(AbstractCharacter::Codepoint('a'), "a".into());
        assert_eq!(AbstractCharacter::Codepoint('⏳'), "⏳".into());
        assert_eq!(
            AbstractCharacter::Variation('\u{1F5E3}', VariantSelector::Emoji),
            "🗣️".into()
        );
        assert_eq!(
            AbstractCharacter::KeycapSequence(Keycap::Digit6),
            "6️⃣".into()
        );
        assert_eq!(
            AbstractCharacter::FlagSequence(Flag::new(
                RegionalIndicator::LetterU,
                RegionalIndicator::LetterN
            )),
            "🇺🇳".into()
        );
        assert_eq!(
            AbstractCharacter::EmojiModifierSequence('🧛', EmojiModifier::Type6),
            "🧛🏿".into()
        );
        assert_eq!(
            AbstractCharacter::EmojiZWJSequence("🏳️‍⚧️".to_string()),
            "🏳️‍⚧️".into()
        );
    }
}
