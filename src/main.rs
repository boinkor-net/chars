extern crate unicode_names;
extern crate byteorder;

use std::env;
use std::fmt;
use std::str;
use std::char;

use byteorder::{ByteOrder, BigEndian};

struct Describable {
    c: char,
}

impl fmt::Display for Describable {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let cp : Codepoint = self.c.into();
        try!(cp.fmt(f));
        let printable : Printable = self.c.into();
        try!(write!(f, "\n{}", printable));
        match unicode_names::name(self.c) {
            Some(n) => {
                write!(f, "\nUnicode name: {}\n", n)
            },
            None => write!(f, "\n")
        }
    }
}

impl std::convert::From<char> for Describable {
    fn from(c: char) -> Describable {
        Describable{c: c}
    }
}

struct Printable {
    c: char,
}

impl std::convert::From<char> for Printable {
    fn from(c: char) -> Printable {
        Printable{c: c}
    }
}


impl fmt::Display for Printable {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let quote : String = self.c.escape_default().collect();
        if self.c.is_control() {
            try!(write!(f, "Control character; quotes as {}", quote));
        } else {
            if ! self.c.is_whitespace() {
                try!(write!(f, "Prints as {}", self.c));
            } else {
                try!(write!(f, "Prints as `{}'", self.c));
            }
            // Check if we can up/downcase:
            let mut caseflipped = String::new();
            if self.c.is_uppercase() {
                for c in self.c.to_lowercase() {
                    caseflipped.push(c);
                }
                try!(write!(f, "\nUpper case. Downcases to {}", caseflipped));
            } else if self.c.is_lowercase() {
                for c in self.c.to_uppercase() {
                    caseflipped.push(c);
                }
                try!(write!(f, "\nLower case. Upcases to {}", caseflipped));
            }

            // If we have quotable text, print that too:
            if quote.len() > 1 {
                try!(write!(f, "\nQuotes as {}", quote));
            }
        }
        Ok(())
    }
}


enum Codepoint {
    ASCII7bit(char),
    Latin1(char),
    UnicodeBasic(char),
    UnicodeWide(char)
}

impl std::convert::From<char> for Codepoint {
    fn from(c: char) -> Codepoint {
        match c as u32 {
            0 ... 128 => Codepoint::ASCII7bit(c),
            128 ... 256 => Codepoint::Latin1(c),
            256 ... 65536 => Codepoint::UnicodeBasic(c),
            _ => Codepoint::UnicodeWide(c),
        }
    }
}

impl fmt::Display for Codepoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Codepoint::ASCII7bit(c) => {
                let num = c as u32;
                write!(f, "ASCII  {:02x}, {:3}, 0x{:02x}, 0{:03o}, bits {:08b}",
                       num, num, num, num, num)
            }
            &Codepoint::Latin1(c) => {
                let num = c as u32;
                write!(f, "LATIN1 {:02x}, {:3}, 0x{:02x}, 0{:03o}, bits {:08b}",
                       num, num, num, num, num)
            }
            &Codepoint::UnicodeBasic(c) | &Codepoint::UnicodeWide(c) => {
                let num = c as u32;
                let mut string = String::new();
                string.push(c);
                let s = string.as_str();
                let utf8 = ByteRepresentation::from(s.bytes());
                let utf16 = ByteRepresentation::from(s.encode_utf16());
                let width = match self {
                    &Codepoint::UnicodeWide(_) => 8,
                    _ => 4,
                };
                write!(f, "U+{:0width$X}, &#{:}; 0x{:0width$X}, \\0{:o}, UTF-8: {}, UTF-16BE: {}",
                       num, num, num, num, utf8, utf16, width = width)
            }
        }
    }
}

enum ByteRepresentation {
    UTF8(Vec<u8>),
    UTF16BE(Vec<u8>),
}

impl<'a> std::convert::From<str::EncodeUtf16<'a>> for ByteRepresentation {
    fn from(bs: str::EncodeUtf16<'a>) -> ByteRepresentation {
        let words: Vec<u16> = bs.collect();
        let mut buf: Vec<u8> = Vec::with_capacity(words.len() * 2);
        for word in words {
            let mut split_word = [0; 2];
            BigEndian::write_u16(&mut split_word, word);
            buf.extend_from_slice(&split_word);
        }

        ByteRepresentation::UTF16BE(buf)
    }
}

impl<'a> std::convert::From<str::Bytes<'a>> for ByteRepresentation {
    fn from(bs: str::Bytes<'a>) -> ByteRepresentation {
        let bytes: Vec<u8> = bs.collect();
        ByteRepresentation::UTF8(bytes)
    }
}

impl fmt::Display for ByteRepresentation {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &ByteRepresentation::UTF8(ref bytes) => {
                let mut byte_iter = bytes.iter();
                try!(write!(f, "{:02x}", byte_iter.next().unwrap()));
                for byte in byte_iter {
                    try!(write!(f, " {:02x}", byte));
                }
            },
            &ByteRepresentation::UTF16BE(ref bytes) => {
                for byte in bytes.iter() {
                    try!(write!(f, "{:02x}", byte));
                }
            }
        }
        Ok(())
    }
}

/// Takes a stringly description of a character (the character itself,
/// or a unicode code point name) and returns a Vec of Describable
/// elements that hold the corresponding character. The elements of
/// the vector are sorted by descending numeric code point.
fn from_arg(spec: &str) -> Vec<Describable> {
    let mut chars: Vec<char> = Vec::new();

    // match the character itself, or any of its names:
    if spec.chars().count() == 1 {
        spec.chars().next().map(|c| chars.push(c));
    } else {
        unicode_names::character(spec).map(|c| chars.push(c));
    }
    // Match hex/U+ strings specifically:
    if spec.starts_with("0x") || spec.starts_with("U+") {
        let _ = u32::from_str_radix(&spec[2..], 16).ok().
            map(|num| char::from_u32(num).map(|c| chars.push(c)));
    }

    // Match plain numbers in all bases:
    for base in vec![16, 10, 8, 2] {
        let _ = u32::from_str_radix(spec, base.clone()).ok().
            map(|num| char::from_u32(num).map(|c| chars.push(c)));
    }
    chars.sort_by(|a, b| b.cmp(a));
    chars.dedup();
    chars.iter().map(|c| c.clone().into()).collect()
}

fn main() {
    let args =
        env::args().skip(1)
        .flat_map(|argument| from_arg(argument.as_ref()));
    for c in args {
        println!("{}", c);
    }
}

#[test]
fn from_arg_translates_chars() {
    assert_eq!('n', from_arg("n")[0].c);
    assert_eq!(']', from_arg("]")[0].c);
}

#[test]
fn from_arg_translates_descriptions() {
    assert_eq!('n', from_arg("latin small letter n")[0].c);
    assert_eq!(']', from_arg("right square bracket")[0].c);
}

#[test]
fn from_arg_translates_numbers() {
    let received = from_arg("60");
    let mut iter = received.iter();
    assert_eq!('`', iter.next().unwrap().c);
    assert_eq!('<', iter.next().unwrap().c);
    assert_eq!('0', iter.next().unwrap().c);

    assert_eq!(2, from_arg("0").len());
    assert_eq!(0x30, from_arg("0").iter().next().unwrap().c as u32);

    assert_eq!(1, from_arg("0x0").len());
    assert_eq!(1, from_arg("0x41").len());
    assert_eq!('A', from_arg("0x41")[0].c);
}
