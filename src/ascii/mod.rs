include!(concat!(env!("OUT_DIR"), "/ascii/names.rs"));

///Return any additional names that are known, accorting to the ascii
///name table.
pub fn additional_names(ch: char) -> Option<Information> {
    let code = ch as usize;
    if code < 128 {
        Some(PRINTABLE_CHARS[code].clone())
    } else {
        None
    }
}
