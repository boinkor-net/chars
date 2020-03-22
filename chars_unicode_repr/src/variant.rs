//! Variant selection.

use std::char::from_u32_unchecked;
use std::fmt;

/// A selection of variant selectors.
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
    VariationSelector(u8),
}

impl Into<char> for &VariantSelector {
    fn into(self) -> char {
        use VariantSelector::*;
        match self {
            Mongolian(vs) => vs.into(),
            Generic(vs) => vs.into(),
            Text => '\u{FE0E}',
            Emoji => '\u{FE0F}',
            VariationSelector(n) => {
                let codepoint = '\u{E0100}' as u32 + (*n as u32);
                unsafe { from_u32_unchecked(codepoint) }
            }
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
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MongolianVariant {
    VS1 = 0x180B,
    VS2 = 0x180C,
    VS3 = 0x180D,
}

impl Into<char> for &MongolianVariant {
    fn into(self) -> char {
        unsafe { from_u32_unchecked(*self as u32) }
    }
}

/// The (generic) Variant Selector (`FE00` through `FE0D`).
///
/// Unicode specifies sequences that use them in
/// [StandardizedVariants.txt](https://unicode.org/Public/UCD/latest/ucd/StandardizedVariants.txt).
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GenericVariant {
    VS1 = 0xFE00,
    VS2 = 0xFE01,
    VS3 = 0xFE02,
    VS4 = 0xFE03,
    VS5 = 0xFE04,
    VS6 = 0xFE05,
    VS7 = 0xFE06,
    VS8 = 0xFE07,
    VS9 = 0xFE08,
    VS10 = 0xFE09,
    VS11 = 0xFE0A,
    VS12 = 0xFE0B,
    VS13 = 0xFE0C,
    VS14 = 0xFE0D,
}

impl Into<char> for &GenericVariant {
    fn into(self) -> char {
        unsafe { from_u32_unchecked(*self as u32) }
    }
}
