use std::char;

use ascii;
use unicode_names;


/// Takes a stringly description of a character (the character itself,
/// or a unicode code point name) and returns a Vec of Describable
/// elements that hold the corresponding character. The elements of
/// the vector are sorted by descending numeric code point.
pub fn from_arg(spec: &str) -> Vec<char> {
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

    // Match ^-escapes as control characters
    if spec.len() == 2 && spec.starts_with("^") {
        let control = spec.as_bytes()[1];
        match control {
            0x3f => chars.push(0x7f as char), // ^? is DEL
            _ => chars.push((spec.as_bytes()[1] & 0x1f) as char)
        }
    }

    // Match characters by ascii(1) name / alias:
    if let Some(ch) = ascii::lookup_by_name(spec) {
        chars.push(ch);
    }

    chars.sort_by(|a, b| b.cmp(a));
    chars.dedup();
    chars
}

#[test]
fn from_arg_translates_chars() {
    assert_eq!('n', from_arg("n")[0]);
    assert_eq!(']', from_arg("]")[0]);
}

#[test]
fn from_arg_translates_descriptions() {
    assert_eq!('n', from_arg("latin small letter n")[0]);
    assert_eq!(']', from_arg("right square bracket")[0]);
}

#[test]
fn from_arg_translates_numbers() {
    let received = from_arg("60");
    let mut iter = received.iter();
    assert_eq!('`', *iter.next().unwrap());
    assert_eq!('<', *iter.next().unwrap());
    assert_eq!('0', *iter.next().unwrap());

    assert_eq!(2, from_arg("0").len());
    assert_eq!(0x30 as char, *from_arg("0").iter().next().unwrap());

    assert_eq!(1, from_arg("0x0").len());
    assert_eq!(1, from_arg("0x41").len());
    assert_eq!('A', from_arg("0x41")[0]);
}

#[test]
fn from_arg_translates_controls() {
    assert_eq!(0x7f as char, from_arg("^?")[0]);
    assert_eq!(0x03 as char, from_arg("^c")[0]);
    assert_eq!(0x03 as char, from_arg("^C")[0]);
}
