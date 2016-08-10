use std::char;
use std::collections::BTreeMap;
use std::num::ParseIntError;
use std::path::Path;
use std::fs::File;
use std::io;
use std::io::{BufReader,BufRead,BufWriter,Write};

#[cfg(test)] use std::iter::FromIterator;
#[cfg(test)] use std::collections::BTreeSet;


use fst_generator;
use fst;
use fst::{MapBuilder};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Format(err: ParseIntError) {
            from()
            cause(err)
            description("Hex number format error")
        }
        IO(err: io::Error) {
            from()
                cause(err)
                description("Couldn't read from file")
        }
        FST(err: fst::Error) {
            from()
                cause(err)
                description("Something went wrong inserting into the FST")
        }
    }
}

fn process_line(names: &mut fst_generator::Names, line: &str) -> Result<bool, Error> {
    if line.starts_with("#") || line.trim_left() == "" {
        return Ok(false);
    }
    let fields: Vec<&str> = line.splitn(3, ";").collect();
    let cp = try!(u32::from_str_radix(fields[0], 16));
    if let Some(ch) = char::from_u32(cp) {
        let query = fields[1].to_owned();
        names.insert(vec![query], ch);
    }
    Ok(true)
}

#[test]
fn test_processing() {
    let mut sorted_names = fst_generator::Names::new();

    // Non-data gets skipped:
    assert!(!process_line(&mut sorted_names, "# this is a comment").unwrap());
    assert!(!process_line(&mut sorted_names, "").unwrap());
    assert!(!process_line(&mut sorted_names, "    ").unwrap());

    // Some from NameAliases.txt:
    assert!(process_line(&mut sorted_names, "0091;PRIVATE USE ONE;control").unwrap());
    assert!(process_line(&mut sorted_names, "0092;PRIVATE USE TWO;control").unwrap());

    assert!(process_line(&mut sorted_names, "0005;ENQUIRY;control").unwrap());
    assert!(process_line(&mut sorted_names, "200D;ZWJ;abbreviation").unwrap());

    // And some from UnicodeData.txt:
    assert!(process_line(&mut sorted_names,
                         "00AE;REGISTERED SIGN;So;0;ON;;;;;N;REGISTERED TRADE MARK SIGN;;;;")
        .unwrap());
    assert!(process_line(&mut sorted_names,
                         "0214;LATIN CAPITAL LETTER U WITH DOUBLE GRAVE;Lu;0;L;0055 \
                          030F;;;;N;;;;0215;e")
        .unwrap());

    let mut iter = sorted_names.iter();
    assert_eq!(iter.next(), Some((&"capital".to_owned(), &BTreeSet::from_iter(vec!['\u{0214}']))));
    assert_eq!(iter.next(), Some((&"double".to_owned(), &BTreeSet::from_iter(vec!['\u{0214}']))));
    assert_eq!(iter.next(), Some((&"enquiry".to_owned(), &BTreeSet::from_iter(vec!['\u{0005}']))));
    assert_eq!(iter.next(), Some((&"grave".to_owned(), &BTreeSet::from_iter(vec!['\u{0214}']))));
    assert_eq!(iter.next(), Some((&"latin".to_owned(), &BTreeSet::from_iter(vec!['\u{0214}']))));
    assert_eq!(iter.next(), Some((&"latin capital letter u with double grave".to_owned(), &BTreeSet::from_iter(vec!['\u{0214}']))));
    assert_eq!(iter.next(), Some((&"one".to_owned(), &BTreeSet::from_iter(vec!['\u{91}']))));
    assert_eq!(iter.next(), Some((&"private".to_owned(), &BTreeSet::from_iter(vec!['\u{91}', '\u{92}']))));
    assert_eq!(iter.next(), Some((&"private use one".to_owned(), &BTreeSet::from_iter(vec!['\u{91}']))));
    assert_eq!(iter.next(), Some((&"private use two".to_owned(), &BTreeSet::from_iter(vec!['\u{92}']))));
    assert_eq!(iter.next(), Some((&"registered".to_owned(), &BTreeSet::from_iter(vec!['\u{AE}']))));
    assert_eq!(iter.next(), Some((&"registered sign".to_owned(), &BTreeSet::from_iter(vec!['\u{AE}']))));
    assert_eq!(iter.next(), Some((&"two".to_owned(), &BTreeSet::from_iter(vec!['\u{92}']))));
    // Skip the "U": it's too short to be meaningful
    assert_eq!(iter.next(), Some((&"use".to_owned(), &BTreeSet::from_iter(vec!['\u{91}', '\u{92}']))));
    assert_eq!(iter.next(), Some((&"zwj".to_owned(), &BTreeSet::from_iter(vec!['\u{200D}']))));
}

pub fn read_names(names: &mut fst_generator::Names, file: &Path) -> Result<(), Error> {
    let reader = BufReader::new(try!(File::open(file)));
    for line in reader.lines() {
        try!(process_line(names, try!(line).as_str()));
    }
    Ok(())
}

pub fn write_name_data(names: &fst_generator::Names, output: &Path) -> Result<(), Error> {
    let fst_byte_filename = output.join("name_fst.bin");
    let out = BufWriter::new(try!(File::create(fst_byte_filename)));
    let mut map_builder = try!(MapBuilder::new(out));

    let mut counter: u64 = 0;
    let mut results: BTreeMap<Vec<&char>, u64> = BTreeMap::new();

    for (name, chs) in names.iter() {
        if chs.len() > 1 {
            let key: Vec<&char> = chs.iter().collect();

            let num: u64 = if results.contains_key(&key) {
                *results.get(&key).unwrap()
            } else {
                counter += 1;
                results.insert(key, counter);
                counter
            };
            let num: u64 = num | (0xff << 32);
            try!(map_builder.insert(name, num));
        } else {
            try!(map_builder.insert(name, *chs.iter().next().unwrap() as u64));
        }
    }
    try!(map_builder.finish());

    // Now generate the multi-results file:
    let multi_result_filename = output.join("names.rs");
    let mut rust_out = BufWriter::new(try!(File::create(multi_result_filename)));
    let mut ambiguous_chars: Vec<Vec<&char>> = vec![Vec::new(); counter as usize + 1];
    for (chars, i) in results {
        ambiguous_chars[i as usize] = chars;
    }
    try!(write!(&mut rust_out, "/// Generated with `make names`\n"));
    try!(write!(&mut rust_out, "pub static AMBIGUOUS_CHARS: &'static [&'static [char]] = &[\n"));
    for chars in ambiguous_chars {
        try!(write!(&mut rust_out, "    &{:?},\n", chars.as_slice()));
    }
    try!(write!(&mut rust_out, "];\n"));

    Ok(())
}
