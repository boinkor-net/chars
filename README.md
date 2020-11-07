# Cha(rs)
[![Build Status](https://travis-ci.org/antifuchs/chars.svg?branch=master)](https://travis-ci.org/antifuchs/chars)

Use this tool to display names and codes for various ASCII (and
unicode) characters / code points!

It's strongly inspired by
[`ascii(1)`](http://www.catb.org/esr/ascii/), but supports unicode
characters; it's also inspired by
[`unicode.py`](http://kassiopeia.juls.savba.sk/~garabik/software/unicode/),
but it attempts to support whitespace/control characters better.

Cha(rs) is currently probably failing at some other edge case, but I
hope not.

## Pronunciation

How do you pronounce "chars"? This is a contentious thing.

## Installation

This package is tested on [circle
CI](https://circleci.com/gh/antifuchs/chars/tree/master) using the
latest stable, beta and nightly releases. Older releases might work,
but I'm focusing development mostly on the latest versions.

### Plain crate installation without source code

`cargo install chars --git https://github.com/antifuchs/chars.git`

### OS packages

**Arch linux:** There's an [AUR package for chars](https://aur.archlinux.org/packages/chars/).

**MacOS** There's a [Homebrew package for chars](https://formulae.brew.sh/formula/chars#default).

**Windows:** There's a package available through [Chocolatey](https://chocolatey.org/packages/chars).

### Source installation
1. Clone this repo,
2. `cd` into the checkout,
3. `cargo install --path chars`

## Running

Look up a character by its face value:

`chars 'ß'`

Screenshot:
```
LATIN1 df, 223, 0xdf, 0337, bits 11011111
Width: 1 (2 in CJK context), prints as ß
Lower case. Upcases to SS
Quotes as \u{df}
Unicode name: LATIN SMALL LETTER SHARP S
```

Look up a character by its unicode point:

`chars U+1F63C`

Screenshot:
```
U+0001F63C, &#128572; 0x0001F63C, \0373074, UTF-8: f0 9f 98 bc, UTF-16BE: d83dde3c
Width: 1, prints as 😼
Quotes as \u{1f63c}
Unicode name: CAT FACE WITH WRY SMILE
```

Look up a character by ambiguous "char code" handwaving:

`chars 10`

Screenshot:
```
U+0001F0EA, &#127210; 0x0001F0EA, \0370352, UTF-8: f0 9f 83 aa, UTF-16BE: d83cdcea
Width: 1, prints as 🃪
Quotes as \u{1f0ea}
Unicode name: PLAYING CARD TRUMP-10

U+0001DAA9, &#121513; 0x0001DAA9, \0355251, UTF-8: f0 9d aa a9, UTF-16BE: d836dea9
Width: 0, prints as 𝪩
Quotes as \u{1daa9}
Unicode name: SIGNWRITING ROTATION MODIFIER-10

U+0001D209, &#119305; 0x0001D209, \0351011, UTF-8: f0 9d 88 89, UTF-16BE: d834de09
Width: 1, prints as 𝈉
Quotes as \u{1d209}
Unicode name: GREEK VOCAL NOTATION SYMBOL-10

U+0001D1A4, &#119204; 0x0001D1A4, \0350644, UTF-8: f0 9d 86 a4, UTF-16BE: d834dda4
Width: 1, prints as 𝆤
Quotes as \u{1d1a4}
Unicode name: MUSICAL SYMBOL ORNAMENT STROKE-10

U+FE09, &#65033; 0xFE09, \0177011, UTF-8: ef b8 89, UTF-16BE: fe09
Width: 0, prints as ︉
Quotes as \u{fe09}
Unicode name: VARIATION SELECTOR-10

ASCII 1/0,  16, 0x10, 0020, bits 00010000
Control character; quotes as \u{10}, called ^P
Called: DLE
Also known as: Data Link Escape

ASCII 0/a,  10, 0x0a, 0012, bits 00001010
Control character; quotes as \n, called ^J
Called: LF, NL
Also known as: Line Feed, Newline, \n

ASCII 0/8,   8, 0x08, 0010, bits 00001000
Control character; quotes as \u{8}, called ^H
Called: BS
Also known as: Backspace, \b

ASCII 0/2,   2, 0x02, 0002, bits 00000010
Control character; quotes as \u{2}, called ^B
Called: STX
Also known as: Start of Text
```

Look a control character:

`chars "^C"`

Screenshot:
```
ASCII 0/3,   3, 0x03, 0003, bits 00000011
Control character; quotes as \u{3}, called ^C
Called: ETX
Also known as: End of Text
```
