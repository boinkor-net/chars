mod names;

///Search for a mnemonic, synonym or other name in the ascii name
///table and return any one that matches.
pub fn lookup_by_name(name: &str) -> Option<char> {
    for &(ch_name, ch) in names::NAMES {
        if name.to_lowercase() == ch_name.to_lowercase() {
            return Some(ch);
        }
    }
    None
}

pub fn additional_names(ch: char) -> Option<names::Information> {
    let code = ch as usize;
    if code < 128 {
        Some(names::PRINTABLE_CHARS[code].clone())
    } else {
        None
    }
}
