//! Variant selection.

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::char::from_u32_unchecked;
use std::convert::TryFrom;
use std::fmt;
use std::ops::RangeInclusive;

/// A selection of variant selectors and modifiers.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VariantSelector {
    /// Mongolian Free Variant Selector 1 (`180B`) through 3 (`180D`).
    Mongolian(MongolianVariant),

    /// (Generic) Variant selectors 1 (`FE00`) through 14 (`FE0D`).
    ///
    /// See the [chart](https://www.unicode.org/charts/PDF/UFE00.pdf)
    /// for more details of the variant selector range and the
    /// [variant sequence FAQ](http://unicode.org/faq/vs.html).
    ///
    /// Note that due to their relative importance in the modern
    /// world, [`VariantSelector::Text`] and
    /// [`VariantSelector::Emoji`] have been pulled out into the
    /// VariantSelector enum.
    Generic(GenericVariant),

    /// The "text" variant selector, `FE0E`. Used to request the "text"
    /// variant of an emoji character. See [UTS
    /// #51](http://www.unicode.org/reports/tr51/tr51-16.html).
    Text,

    /// The "emoji" variant selector, `FE0F`.
    Emoji,

    /// Variation Selectors 1 (`E0100`) through 256 (`E01FF`). Used
    /// for the [IVD sequence
    /// database](https://unicode.org/ivd/data/2017-12-12/IVD_Sequences.txt),
    /// see also [Unicode tr37](http://www.unicode.org/reports/tr37/).
    ///
    /// This is a confusing name, but it is the name that unicode specifies.
    VariationSelector(u8),
}

impl VariantSelector {
    pub(crate) const TEXT_SELECTOR: char = '\u{FE0E}';
    pub(crate) const EMOJI_SELECTOR: char = '\u{FE0F}';
    pub(crate) const VARIATION_SELECTOR_RANGE: RangeInclusive<u32> = 0xE0100..=0xE01FF;
}

impl Into<char> for &VariantSelector {
    fn into(self) -> char {
        use VariantSelector::*;
        match self {
            Mongolian(vs) => vs.into(),
            Generic(vs) => vs.into(),
            Text => VariantSelector::TEXT_SELECTOR,
            Emoji => VariantSelector::EMOJI_SELECTOR,
            VariationSelector(n) => {
                let codepoint = '\u{E0100}' as u32 + (*n as u32);
                unsafe { from_u32_unchecked(codepoint) }
            }
        }
    }
}

impl TryFrom<char> for VariantSelector {
    /// The original char that is not a known variant selector:
    type Error = char;

    fn try_from(f: char) -> Result<Self, Self::Error> {
        if let Ok(v) = MongolianVariant::try_from(f) {
            Ok(VariantSelector::Mongolian(v))
        } else if let Ok(v) = GenericVariant::try_from(f) {
            Ok(VariantSelector::Generic(v))
        } else if f == VariantSelector::TEXT_SELECTOR {
            Ok(VariantSelector::Text)
        } else if f == VariantSelector::EMOJI_SELECTOR {
            Ok(VariantSelector::Emoji)
        } else if VariantSelector::VARIATION_SELECTOR_RANGE.contains(&(f as u32)) {
            let offset = VariantSelector::VARIATION_SELECTOR_RANGE.start() - (f as u32);
            Ok(VariantSelector::VariationSelector(offset as u8))
        } else {
            Err(f)
        }
    }
}

impl fmt::Display for VariantSelector {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let c: char = self.into();
        write!(f, "{}", c)
    }
}

/// Mongolian variant selectors.
///
/// Unicode specifies sequences that use them in
/// [StandardizedVariants.txt](https://unicode.org/Public/UCD/latest/ucd/StandardizedVariants.txt).
#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
pub enum MongolianVariant {
    VS1 = 0x180B,
    VS2,
    VS3,
}

impl Into<char> for &MongolianVariant {
    fn into(self) -> char {
        unsafe { from_u32_unchecked(*self as u32) }
    }
}

impl TryFrom<char> for MongolianVariant {
    /// The original char that is not a mongolian variant selector
    type Error = char;

    fn try_from(f: char) -> Result<Self, Self::Error> {
        MongolianVariant::from_u32(f as u32).ok_or(f)
    }
}

/// The (generic) Variant Selector (`FE00` through `FE0D`).
///
/// Unicode specifies sequences that use them in
/// [StandardizedVariants.txt](https://unicode.org/Public/UCD/latest/ucd/StandardizedVariants.txt).
#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
pub enum GenericVariant {
    VS1 = 0xFE00,
    VS2,
    VS3,
    VS4,
    VS5,
    VS6,
    VS7,
    VS8,
    VS9,
    VS10,
    VS11,
    VS12,
    VS13,
    VS14,
}

impl Into<char> for &GenericVariant {
    fn into(self) -> char {
        unsafe { from_u32_unchecked(*self as u32) }
    }
}

impl TryFrom<char> for GenericVariant {
    /// The original char that is not a generic variant selector
    type Error = char;

    fn try_from(f: char) -> Result<Self, Self::Error> {
        GenericVariant::from_u32(f as u32).ok_or(f)
    }
}
