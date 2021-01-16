use std::char;
use std::collections::BTreeMap;
use std::fs::{create_dir_all, File};
use std::io;
use std::io::{BufRead, BufWriter, Cursor, Write};
use std::num::ParseIntError;
use std::path::Path;

#[cfg(test)]
use std::collections::BTreeSet;
#[cfg(test)]
use std::iter::FromIterator;

use crate::fst_generator;
use fst::MapBuilder;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Format(err: ParseIntError) {
            from()
            cause(err)
            description("Hex number format error")
        }
        Block(ch: u32) {
            display("Inconsistent block at {}", ch)
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

#[derive(PartialEq, Debug)]
enum LineType {
    None,
    Simple,
    BlockStart(u32),
    BlockEnd(u32),
}

const NAME_ALIASES: &[u8] = include_bytes!("../data/unicode/NameAliases.txt");
const UNICODE_DATA: &[u8] = include_bytes!("../data/unicode/UnicodeData.txt");

pub(crate) fn name_aliases() -> Cursor<&'static [u8]> {
    Cursor::new(NAME_ALIASES)
}

pub(crate) fn unicode_data() -> Cursor<&'static [u8]> {
    Cursor::new(UNICODE_DATA)
}

fn process_line(names: &mut fst_generator::Names, line: &str) -> Result<LineType, Error> {
    if line.starts_with('#') || line.trim_start() == "" {
        return Ok(LineType::None);
    }
    let fields: Vec<&str> = line.splitn(15, ';').collect();
    let cp = u32::from_str_radix(fields[0], 16)?;
    if let Some(ch) = char::from_u32(cp) {
        let query = fields[1].to_owned();
        if query.ends_with(", First>") {
            return Ok(LineType::BlockStart(ch as u32));
        } else if query.ends_with(", Last>") {
            return Ok(LineType::BlockEnd(ch as u32));
        }
        names.insert(vec![query], ch);
        match fields.get(10) {
            Some(&"") | None => {}
            Some(&name) => names.insert(vec![name.to_string()], ch),
        }
        Ok(LineType::Simple)
    } else {
        Ok(LineType::None)
    }
}

#[test]
fn test_processing() {
    {
        let mut sorted_names = fst_generator::Names::new();

        // Non-data gets skipped:
        assert_eq!(
            LineType::None,
            process_line(&mut sorted_names, "# this is a comment").unwrap()
        );
        assert_eq!(LineType::None, process_line(&mut sorted_names, "").unwrap());
        assert_eq!(
            LineType::None,
            process_line(&mut sorted_names, "    ").unwrap()
        );
    }
    {
        let mut sorted_names = fst_generator::Names::new();
        assert_eq!(
            LineType::Simple,
            process_line(
                &mut sorted_names,
                "03BB;GREEK SMALL LETTER LAMDA;Ll;0;L;;;;;N;GREEK SMALL LETTER LAMBDA;;039B;;039B"
            )
            .unwrap()
        );
        let have = BTreeSet::from_iter(sorted_names.iter().map(|(name, chs)| {
            let v: Vec<char> = chs.iter().map(|ch| ch.to_owned()).collect();
            (name.as_str(), v)
        }));
        let want = BTreeSet::from_iter(vec![
            // Current unicode spelling:
            ("greek", vec!['\u{03bb}']),
            ("greek small letter lamda", vec!['\u{03bb}']),
            ("lamda", vec!['\u{03bb}']),
            // Unicode 1.0 spelling:
            ("greek small letter lambda", vec!['\u{03bb}']),
            ("lambda", vec!['\u{03bb}']),
        ]);
        assert_eq!(have, want);
    }

    {
        let mut sorted_names = fst_generator::Names::new();
        // Some from NameAliases.txt:
        assert_eq!(
            LineType::Simple,
            process_line(&mut sorted_names, "0091;PRIVATE USE ONE;control").unwrap()
        );
        assert_eq!(
            LineType::Simple,
            process_line(&mut sorted_names, "0092;PRIVATE USE TWO;control").unwrap()
        );

        assert_eq!(
            LineType::Simple,
            process_line(&mut sorted_names, "0005;ENQUIRY;control").unwrap()
        );
        assert_eq!(
            LineType::Simple,
            process_line(&mut sorted_names, "200D;ZWJ;abbreviation").unwrap()
        );

        // And some from UnicodeData.txt:
        assert_eq!(
            LineType::Simple,
            process_line(
                &mut sorted_names,
                "00AE;REGISTERED SIGN;So;0;ON;;;;;N;REGISTERED TRADE MARK SIGN;;;;"
            )
            .unwrap()
        );
        assert_eq!(
            LineType::Simple,
            process_line(
                &mut sorted_names,
                "0214;LATIN CAPITAL LETTER U WITH DOUBLE GRAVE;Lu;0;L;0055 \
                 030F;;;;N;;;;0215;e"
            )
            .unwrap()
        );

        // CJK blocks:
        assert_eq!(
            LineType::BlockStart(0x3400),
            process_line(
                &mut sorted_names,
                "3400;<CJK Ideograph Extension A, First>;Lo;0;L;;;;;N;;;;;"
            )
            .unwrap()
        );
        assert_eq!(
            LineType::BlockEnd(0x4DB5),
            process_line(
                &mut sorted_names,
                "4DB5;<CJK Ideograph Extension A, Last>;Lo;0;L;;;;;N;;;;;"
            )
            .unwrap()
        );

        let have = BTreeSet::from_iter(sorted_names.iter().map(|(name, chs)| {
            let v: Vec<char> = chs.iter().map(|ch| ch.to_owned()).collect();
            (name.as_str(), v)
        }));
        let want = BTreeSet::from_iter(vec![
            ("capital", vec!['\u{0214}']),
            ("double", vec!['\u{0214}']),
            ("enquiry", vec!['\u{0005}']),
            ("grave", vec!['\u{0214}']),
            ("latin", vec!['\u{0214}']),
            ("latin capital letter u with double grave", vec!['\u{0214}']),
            ("one", vec!['\u{91}']),
            ("two", vec!['\u{92}']),
            ("private", vec!['\u{91}', '\u{92}']),
            ("use", vec!['\u{91}', '\u{92}']),
            ("private use one", vec!['\u{91}']),
            ("private use two", vec!['\u{92}']),
            ("registered", vec!['\u{AE}']),
            ("trade", vec!['\u{AE}']),
            ("mark", vec!['\u{AE}']),
            ("registered sign", vec!['\u{AE}']),
            ("registered trade mark sign", vec!['\u{AE}']),
            // Skip the "U": it's too short to be meaningful
            ("zwj", vec!['\u{200D}']),
        ]);
        assert_eq!(have, want);
    }
}

#[test]
fn test_old_names() {}

pub fn read_names(names: &mut fst_generator::Names, reader: impl BufRead) -> Result<(), Error> {
    let mut lines = reader.lines();
    while let Some(line) = lines.next() {
        match process_line(names, line?.as_str())? {
            LineType::Simple | LineType::None => {}
            LineType::BlockStart(start) => {
                let line = lines.next().ok_or(Error::Block(start))??;
                match process_line(names, &line)? {
                    LineType::Simple | LineType::None | LineType::BlockStart(_) => {
                        return Err(Error::Block(start));
                    }
                    LineType::BlockEnd(end) => {
                        for i in start..=end {
                            if let Some(ch) = char::from_u32(i as u32) {
                                if let Some(name) = unicode_names2::name(ch) {
                                    names.insert(vec![name.to_string()], ch);
                                }
                            }
                        }
                    }
                }
            }
            LineType::BlockEnd(end) => {
                return Err(Error::Block(end));
            }
        }
    }
    Ok(())
}

pub fn write_name_data(names: &fst_generator::Names, output: &Path) -> Result<(), Error> {
    create_dir_all(output)?;
    let fst_byte_filename = output.join("name_fst.bin");
    let out = BufWriter::new(File::create(fst_byte_filename)?);
    let mut map_builder = MapBuilder::new(out)?;

    let mut counter: u64 = 0;
    let mut results: BTreeMap<String, u64> = BTreeMap::new();

    for (name, chs) in names.iter() {
        if chs.len() > 1 {
            let mut key = String::new();
            for c in chs {
                key.push(*c)
            }

            let num: u64 = (0xff << 32)
                | *results.entry(key).or_insert_with(|| {
                    counter += 1;
                    counter - 1
                });
            map_builder.insert(name, num)?;
        } else {
            map_builder.insert(name, *chs.iter().next().unwrap() as u64)?;
        }
    }
    map_builder.finish()?;

    // Now generate the multi-results file:
    let multi_result_filename = output.join("names.rs");
    let mut rust_out = BufWriter::new(File::create(multi_result_filename)?);
    let mut ambiguous_chars: Vec<String> = vec![String::new(); counter as usize];
    for (chars, i) in results {
        ambiguous_chars[i as usize] = chars;
    }
    writeln!(&mut rust_out, "/// Generated with `make names`")?;
    writeln!(&mut rust_out, "#[rustfmt::skip]")?;
    writeln!(
        &mut rust_out,
        "pub static AMBIGUOUS_CHARS: &[&str; {}] = &[",
        ambiguous_chars.len()
    )?;
    for chars in ambiguous_chars {
        writeln!(&mut rust_out, "    {:?},", chars)?;
    }
    writeln!(&mut rust_out, "];")?;

    Ok(())
}
