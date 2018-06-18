#[macro_use]
extern crate proptest;
extern crate chars;

use chars::human_names;

use proptest::prelude::*;
use std::fmt::Write;

proptest! {
    #[test]
    fn find_any_by_identity(ch in prop::char::any()) {
        let mut chstr = String::new();
        chstr.write_char(ch).unwrap();

        let found = human_names::from_arg(&chstr);
        assert!(found.len() >= 1);
        assert!(found.contains(&ch));
    }

    #[test]
    fn find_any_by_hex(ch in prop::char::any()) {
        let num = ch as u32;
        let found = human_names::from_arg(&format!("0x{:04x}", num));
        println!("num: {:?}", num);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0], ch);

        let found = human_names::from_arg(&format!("U+{:04x}", num));
        assert_eq!(found.len(), 1);
        assert_eq!(found[0], ch);

        let found = human_names::from_arg(&format!("{:04x}", num));
        assert!(found.len() >= 1);
        assert!(found.contains(&ch));
    }

    #[test]
    fn find_control_chars(ch in prop::char::range(0 as char, 0x1f as char)) {
        let query = format!("^{}", (b'@' + (ch as u8 & 0x1f)) as char);
        let found = human_names::from_arg(&query);
        assert_eq!(found.len(), 1, "nothing found for query: {:?}", query);
        assert_eq!(found[0], ch, "query: {:?}", query);
    }
}
