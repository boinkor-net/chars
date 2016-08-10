mod names;

///Return any additional names that are known, accorting to the ascii
///name table.
pub fn additional_names(ch: char) -> Option<names::Information> {
    let code = ch as usize;
    if code < 128 {
        Some(names::PRINTABLE_CHARS[code].clone())
    } else {
        None
    }
}
