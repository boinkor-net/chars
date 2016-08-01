extern crate unicode_names;

use std::env;
use std::fmt;

struct Describable {
    c: char,
}

impl fmt::Display for Describable {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match unicode_names::name(self.c) {
            Some(n) =>  write!(f, "{} = {}", self.c, n),
            None => write!(f, "{} is unknown", self.c)
        }
    }
}

/// Takes a stringly description of a character (the character itself,
/// or a unicode code point name) and returns either None (if the
/// character description is not understood), or Some(Describable)
/// that holds the character.
fn from_arg(spec: &str) -> Option<Describable> {
    if spec.chars().count() == 1 {
        spec.chars().next().map(|c| Describable{c: c})
    } else {
        unicode_names::character(spec).map(|c| Describable{c: c})
    }
    // TODO: \u, 0x, 0-prefixed chars, decimals too
    // TODO: Return multiple chars if there's ambiguity.
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
    assert_eq!('c', from_arg("c").unwrap().c);
    assert_eq!(']', from_arg("]").unwrap().c);
}

#[test]
fn from_arg_translates_descriptions() {
    assert_eq!('c', from_arg("latin small letter c").unwrap().c);
    assert_eq!(']', from_arg("right square bracket").unwrap().c);
}
