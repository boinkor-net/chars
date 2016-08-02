extern crate unicode_names;

use std::env;
use std::fmt;
use std::char;

struct Describable {
    c: char,
}

impl fmt::Display for Describable {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let cp : Codepoint = self.c.into();
        try!(cp.fmt(f));
        let quote : String = self.c.escape_default().collect();
        try!(write!(f, ": prints as {}", quote));
        match unicode_names::name(self.c) {
            Some(n) => {
                write!(f, "\nUnicode name: {} = {}\n",
                       self.c, n)
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

enum Codepoint {
    ASCII7bit(char),
    Latin1(char),
    Unicode(char)
}

impl std::convert::From<char> for Codepoint {
    fn from(c: char) -> Codepoint {
        match c as u32 {
            0 ... 128 => Codepoint::ASCII7bit(c),
            128 ... 256 => Codepoint::Latin1(c),
            _ => Codepoint::Unicode(c),
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
                write!(f, "LATIN1 {:x}, {:}, 0x{:x}, 0{:o}, bits {:b}",
                       num, num, num, num, num)
            }
            &Codepoint::Unicode(c) => {
                let num = c as u32;
                write!(f, "UCS 4  {:x}, {:}, 0x{:x}, 0{:o}, bits {:b}",
                       num, num, num, num, num)
            }
        }
    }
}

/// Takes a stringly description of a character (the character itself,
/// or a unicode code point name) and returns either None (if the
/// character description is not understood), or Some(Describable)
/// that holds the character.
fn from_arg(spec: &str) -> Vec<Describable> {
    let mut res : Vec<Describable> = Vec::new();

    if spec.chars().count() == 1 {
        spec.chars().next().map(|c| res.push(Describable{c: c}));
    } else {
        unicode_names::character(spec).map(|c| res.push(Describable{c: c}));
    }
    for base in vec![16, 10, 8, 2] {
        let _ = u32::from_str_radix(spec, base.clone()).ok().
            map(|num| char::from_u32(num).map(|c| res.push(Describable{c: c})));
    }
    res
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
    assert_eq!('c', from_arg("c")[0].c);
    assert_eq!(']', from_arg("]")[0].c);
}

#[test]
fn from_arg_translates_descriptions() {
    assert_eq!('c', from_arg("latin small letter c")[0].c);
    assert_eq!(']', from_arg("right square bracket")[0].c);
}

#[test]
fn from_arg_translates_numbers() {
    let received = from_arg("60");
    let mut iter = received.iter();
    assert_eq!('`', iter.next().unwrap().c);
    assert_eq!('<', iter.next().unwrap().c);
    assert_eq!('0', iter.next().unwrap().c);
}
