extern crate unicode_names;

use std::env;
use std::fmt;
use std::char;

struct Describable {
    c: char,
}

impl fmt::Display for Describable {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match unicode_names::name(self.c) {
            // TODO: restructure this for high-bit chars
            Some(n) => {
                let num = self.c as u32;
                let quote : String = self.c.escape_default().collect();
                write!(f, "{} = {}, {:}, 0x{:x}, 0{:o}, bits {:b}: prints as {}",
                       self.c, n, num, num, num, num, quote)
            },
            None => write!(f, "{} is unknown", self.c)
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
    for base in vec![10, 16, 8, 2] {
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
    assert_eq!('<', received[0].c);
    assert_eq!('`', received[1].c);
    assert_eq!('0', received[2].c);
}
