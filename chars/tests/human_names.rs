use chars::human_names;

use proptest::prelude::*;
use std::fmt::Write;

fn diagnostics(ch: char, query: &str) -> String {
    format!(
        "char: {:?} / {}, query: {:?}; maybe `make fetch` needs to be run",
        ch,
        ch.escape_unicode(),
        query
    )
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100_000))]

    #[test]
    fn find_any_by_identity(ch in prop::char::any()) {
        let mut chstr = String::new();
        chstr.write_char(ch).unwrap();

        let found = human_names::from_arg(&chstr);
        assert!(!found.is_empty());
        assert!(found.contains(&ch));
    }

    #[test]
    fn find_by_name(ch in prop::char::any().prop_filter("Must have a name",
                                                        |c| unicode_names2::name(*c).is_some())) {
        let query = unicode_names2::name(ch).unwrap().to_string();

        let found = human_names::from_arg(&query);
        assert!(!found.is_empty(), "{}", diagnostics(ch, &query));
        assert!(found.contains(&ch), "{}", diagnostics(ch, &query));
    }

    #[test]
    fn find_any_by_hex(ch in prop::char::any()) {
        let num = ch as u32;
        let query = format!("0x{:04x}", num);
        let found = human_names::from_arg(&query);
        println!("num: {:?}", num);
        assert_eq!(found.len(), 1, "{}", diagnostics(ch, &query));
        assert_eq!(found[0], ch, "{}", diagnostics(ch, &query));

        let query = format!("U+{:04x}", num);
        let found = human_names::from_arg(&query);
        assert_eq!(found.len(), 1, "{}", diagnostics(ch, &query));
        assert_eq!(found[0], ch, "{}", diagnostics(ch, &query));

        let query = format!("{:04x}", num);
        let found = human_names::from_arg(&query);
        assert!(!found.is_empty(), "{}", diagnostics(ch, &query));
        assert!(found.contains(&ch), "{}", diagnostics(ch, &query));
    }

    #[test]
    fn find_control_chars(ch in prop::char::range(0 as char, 0x1f as char)) {
        let query = format!("^{}", (b'@' + (ch as u8 & 0x1f)) as char);
        let found = human_names::from_arg(&query);
        assert_eq!(found.len(), 1, "nothing found for query: {:?}", query);
        assert_eq!(found[0], ch, "query: {:?}", query);
    }
}
