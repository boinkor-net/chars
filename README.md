# Cha(rs)

Use this tool to display names and codes for various ASCII (and
unicode) characters / code points!

It's strongly inspired by
[`ascii(1)`](http://www.catb.org/esr/ascii/), but supports unicode
characters; it's also inspired by
[`unicode.py`](http://kassiopeia.juls.savba.sk/~garabik/software/unicode/),
but it attempts to support whitespace/control charaters better.

Cha(rs) is currently probably failing at some other edge case, but I
hope not.

## Pronunciation

How do you pronounce "chars"? This is a contended thing.

## Installation

Prereqs: I am building `chars` on Rust 1.10. I have heard reports that
Rust 1.8 can't build this project, so there is a lower bound somewhere
below 1.10.

### Plain crate installation without source code

`cargo install --git https://github.com/antifuchs/chars.git`

### Source installation
1. Clone this repo
2. `cargo install`

## Running

Look up a character by its face value:

`chars 'ß'`

Screenshot:
```
LATIN1 df, 223, 0xdf, 0337, bits 11011111
Prints as ß
Lower case. Upcases to SS
Quotes as \u{df}
Unicode name: LATIN SMALL LETTER SHARP S
```

Look up a character by its unicode point:

`chars U+1F63C`

Screenshot:
```
U+0001F63C, &#128572; 0x0001F63C, \0373074, UTF-8: f0 9f 98 bc, UTF-16BE: d83dde3c
Prints as 😼
Quotes as \u{1f63c}
Unicode name: CAT FACE WITH WRY SMILE
```

Look up a character by ambiguous "char code" handwaving:

`chars 10`

Screenshot:
```
ASCII  10,  16, 0x10, 0020, bits 00010000
Control character; quotes as \u{10}

ASCII  0a,  10, 0x0a, 0012, bits 00001010
Control character; quotes as \n

ASCII  08,   8, 0x08, 0010, bits 00001000
Control character; quotes as \u{8}

ASCII  02,   2, 0x02, 0002, bits 00000010
Control character; quotes as \u{2}
```
