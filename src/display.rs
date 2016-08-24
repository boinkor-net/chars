use std::fmt;
use std::str;
use std::char;
use std::convert;

use byteorder::{ByteOrder, BigEndian};
use unicode_names;
use unicode_width::UnicodeWidthChar;

use super::ascii;

pub fn describe(c: char) {
    println!("{}\n", Describable::from(c));
}

struct Describable {
    c: char,
}

impl fmt::Display for Describable {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let cp : Codepoint = self.c.into();
        try!(cp.fmt(f));
        let printable : Printable = self.c.into();
        try!(write!(f, "\n{}", printable));
        let unicode_name = unicode_names::name(self.c);
        if let Some(n) = unicode_name.clone() {
            try!(write!(f, "\nUnicode name: {}", n));
        }
        if let Some(ascii) = ascii::additional_names(self.c) {
            let mut synonyms: Vec<&str> = vec!();
            let mut xmls: Option<&str> = None;
            let mnemos: Vec<&str> = ascii.mnemonics.iter().filter(|n| n.len() != 1).map(|s| *s).collect();
            for syn in ascii.synonyms {
                if syn.starts_with('&') && syn.ends_with(';') {
                    xmls = Some(syn);
                } else if let Some(unicode) = unicode_name.clone() {
                    if format!("{}", unicode).as_str().to_lowercase() != *syn.to_lowercase() {
                        synonyms.push(syn);
                    }
                } else {
                    synonyms.push(syn);
                }
            }
            if !mnemos.is_empty() {
                try!(write!(f, "\nCalled: {}", mnemos.join(", ")));
            }
            if !synonyms.is_empty() {
                try!(write!(f, "\nAlso known as: {}", synonyms.join(", ")));
            }
            if let Some(xml) = xmls {
                try!(write!(f, "\nEscapes in XML as: {}", xml));
            }
            if let Some(n) = ascii.note {
                try!(write!(f, "\nNote: {}", n));
            }
        }
        Ok(())
    }
}

impl convert::From<char> for Describable {
    fn from(c: char) -> Describable {
        Describable{c: c}
    }
}

struct Printable {
    c: char,
}

impl convert::From<char> for Printable {
    fn from(c: char) -> Printable {
        Printable{c: c}
    }
}

fn control_char(ch: char) -> char {
    match ch as u8 {
        0x7f => '?',
        _ => (b'@' + (ch as u8 & 0x1f)) as char
    }
}

impl fmt::Display for Printable {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let quote : String = self.c.escape_default().collect();
        if self.c.is_control() {
            try!(write!(f, "Control character; quotes as {}, called ^{}", quote, control_char(self.c)));
        } else {
            if let (Some(width), Some(cjk_width)) = (self.c.width(), self.c.width_cjk()) {
                if width == cjk_width {
                    try!(write!(f, "Width: {}, ", width));
                } else {
                    try!(write!(f, "Width: {} ({} in CJK context), ", width, cjk_width));
                }
            }
            if ! self.c.is_whitespace() {
                try!(write!(f, "prints as {}", self.c));
            } else {
                try!(write!(f, "prints as `{}'", self.c));
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

impl convert::From<char> for Codepoint {
    fn from(c: char) -> Codepoint {
        match c as u32 {
            0 ... 127 => Codepoint::ASCII7bit(c),
            128 ... 255 => Codepoint::Latin1(c),
            256 ... 65535 => Codepoint::UnicodeBasic(c),
            _ => Codepoint::UnicodeWide(c),
        }
    }
}

impl fmt::Display for Codepoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Codepoint::ASCII7bit(c) => {
                let num = c as u32;
                write!(f, "ASCII {:1x}/{:1x}, {:3}, 0x{:02x}, 0{:03o}, bits {:08b}",
                       (num & 0xf0) >> 4, num & 0x0f, num, num, num, num)
            }
            Codepoint::Latin1(c) => {
                let num = c as u32;
                write!(f, "LATIN1 {:02x}, {:3}, 0x{:02x}, 0{:03o}, bits {:08b}",
                       num, num, num, num, num)
            }
            Codepoint::UnicodeBasic(c) | Codepoint::UnicodeWide(c) => {
                let num = c as u32;
                let mut string = String::new();
                string.push(c);
                let s = string.as_str();
                let utf8 = ByteRepresentation::from(s.bytes());
                let utf16 = ByteRepresentation::from(s.encode_utf16());
                let width = match *self {
                    Codepoint::UnicodeWide(_) => 8,
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

impl<'a> convert::From<str::EncodeUtf16<'a>> for ByteRepresentation {
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

impl<'a> convert::From<str::Bytes<'a>> for ByteRepresentation {
    fn from(bs: str::Bytes<'a>) -> ByteRepresentation {
        let bytes: Vec<u8> = bs.collect();
        ByteRepresentation::UTF8(bytes)
    }
}

impl fmt::Display for ByteRepresentation {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ByteRepresentation::UTF8(ref bytes) => {
                let mut byte_iter = bytes.iter();
                try!(write!(f, "{:02x}", byte_iter.next().unwrap()));
                for byte in byte_iter {
                    try!(write!(f, " {:02x}", byte));
                }
            },
            ByteRepresentation::UTF16BE(ref bytes) => {
                for byte in bytes.iter() {
                    try!(write!(f, "{:02x}", byte));
                }
            }
        }
        Ok(())
    }
}
