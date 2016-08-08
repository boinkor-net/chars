use std::char;

use fst::Map;

mod names;

const BYTES: &'static [u8] = include_bytes!("name_fst.bin");

pub fn lookup_by_query(query: &str) -> Vec<char> {
    lazy_static! {
        static ref FST: Map = Map::from_bytes(BYTES.to_owned()).unwrap();
    }
    let mut chars = vec!();
    let query = query.to_lowercase();

    if let Some(cp) = FST.get(query) {
        if cp & (0xff<<32) != 0 {
            let index: usize = (cp as u32) as usize;
            for ch in names::AMBIGUOUS_CHARS[index] {
                chars.push(*ch);
            }
        } else {
            char::from_u32(cp as u32).map(|ch| chars.push(ch));
        }
    }
    chars
}
