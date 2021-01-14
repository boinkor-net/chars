use std::char;
use std::collections::BTreeSet;

use fst::Map;

const BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/unicode/name_fst.bin"));
include!(concat!(env!("OUT_DIR"), "/unicode/names.rs"));

fn query_fst(word: &str) -> Vec<char> {
    lazy_static! {
        static ref FST: Map<Vec<u8>> = Map::new(BYTES.to_owned()).unwrap();
    }

    let mut chars: Vec<char> = vec![];
    if let Some(cp) = FST.get(word) {
        if cp & (0xff << 32) != 0 {
            let index: usize = (cp as u32) as usize;
            for ch in AMBIGUOUS_CHARS[index].chars() {
                chars.push(ch);
            }
        } else if let Some(ch) = char::from_u32(cp as u32) {
            chars.push(ch)
        }
    }
    chars
}

pub fn lookup_by_query(query: &str) -> Vec<char> {
    let query = query.to_lowercase();
    // try the original query first:
    let original_results = query_fst(query.as_str());
    if !original_results.is_empty() {
        return original_results;
    }

    let mut candidates = BTreeSet::new();
    if query.contains(char::is_whitespace) {
        // Split multiple-word queries, AND them together:
        let mut words = query.split_whitespace();
        if let Some(word) = words.next() {
            for ch in query_fst(word) {
                candidates.insert(ch);
            }
        }
        for word in words {
            let mut merge_candidates = BTreeSet::new();
            for ch in query_fst(word) {
                merge_candidates.insert(ch);
            }
            candidates = candidates
                .intersection(&merge_candidates)
                .cloned()
                .collect();
            if candidates.is_empty() {
                return vec![];
            }
        }
    }
    candidates.into_iter().collect()
}
