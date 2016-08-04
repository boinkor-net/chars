# Cha(rs)

Use this tool to display names and codes for various ASCII (and
unicode) characters / code points!

## Pronunciation

How do you pronounce "chars"? This is a contended thing.

## Installation

Prereqs: I am building `chars` on Rust 1.10. Rusts at 1.8 and above
should work, but no guarantees.

1. Clone this repo
2. `cargo install`

## Running

Look up a character by its face value:

`chars '#'`

Look up a character by its unicode point:

`chars U+1F63C`

Look up a character by ambiguous "char code" handwaving:

`chars 10`
