#[macro_use]
extern crate proptest;
extern crate chars;

use chars::human_names;
use proptest::prelude::*;
use std::fmt::Write;

proptest! {
#[test]
fn find_any_by_name(ch in prop::char::any()) {
    let mut chstr = String::new();
    chstr.write_char(ch).unwrap();

    let found = human_names::from_arg(&chstr);
    assert!(found.len() >= 1);
    assert!(found.contains(&ch));
}
}
