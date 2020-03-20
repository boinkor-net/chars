///# Parse ascii(1) `nametable` files
///
/// Nametables have an almost deceptively simple format:
///
///```text
///Mnemonics: "\"",
///ISO names: "Quotation Mark",
///Synonyms:  "Double Quote", "Quote", "String Quote",
///            "Dirk", "Literal Mark", "Double Glitch",
///XML name: "&quot;",
///Comment:   "# See ' and ` for matching names.",
///%%
///```
///
/// Any C Strings that are shorter than 4 elements count as "called"
/// abbreviations (think `TAB`, `NL`).  All control chars (in C, also
/// `0x7F`; it counts as a control char in rust) are printed in their
/// ^-notation, as `&`ed with `0x1F` (except again `0x7F`, this is
/// printed as ^?).
///
/// The first non-"called" name is the "Official name", and after is
/// an (optional) C escape and other names, with a Note if any other
/// name starts with a "#".
use std::fmt;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::io::{BufRead, BufWriter, Cursor};
use std::path::Path;

use regex::Regex;

use fst_generator;

#[derive(Debug, Clone)]
struct ASCIIEntry {
    value: char,
    mnemonics: Vec<String>,
    synonyms: Vec<String>,
    note: Option<String>,
}

#[derive(Debug)]
struct ASCIIForDisplay<'a> {
    val: &'a ASCIIEntry,
}

impl ASCIIEntry {
    fn new(code: u8) -> ASCIIEntry {
        ASCIIEntry {
            value: code as char,
            mnemonics: vec![],
            synonyms: vec![],
            note: None,
        }
    }

    fn for_display(&self) -> ASCIIForDisplay {
        ASCIIForDisplay { val: self }
    }
}

fn split_name_line(line: &str) -> Vec<String> {
    lazy_static! {
        static ref QUOTES: Regex = Regex::new("\",\\s*\"").unwrap();
        static ref RIGHT_QUOTE: Regex = Regex::new("\"\\s*,\\s*$").unwrap();
        static ref BACKSLASH_SOMETHING: Regex = Regex::new(r"\\(.)").unwrap();
    }
    let line = line.trim_start();
    let line = RIGHT_QUOTE.replace_all(line, "$1");
    let line = line.trim_start_matches('"');
    QUOTES
        .split(line)
        .map(|s| BACKSLASH_SOMETHING.replace_all(s, "$1").into_owned())
        .collect()
}

#[test]
fn test_split_name_line() {
    assert_eq!(
        split_name_line(r#" "Shift In", "Locking Shift 0","#),
        vec!["Shift In", "Locking Shift 0"]
    );
    assert_eq!(split_name_line(r#""\\v","#), vec![r"\v"]);
    assert_eq!(split_name_line(r#""\"","#), vec![r#"""#]);
}

const NAMETABLE: &[u8] = include_bytes!("../data/ascii/nametable");

fn process_ascii_nametable() -> Result<Vec<ASCIIEntry>, io::Error> {
    let mut char_code: u8 = 0;

    let mut entry = ASCIIEntry::new(char_code);
    let mut entries: Vec<ASCIIEntry> = vec![];

    let reader = Cursor::new(NAMETABLE);
    for line in reader.lines() {
        let line = line?;
        let line = line.as_str();
        match line.chars().next() {
            Some('#') | None => {}
            Some(_) => {
                if line == "%%" {
                    entries.push(entry);
                    char_code += 1;
                    entry = ASCIIEntry::new(char_code);
                    continue;
                }
                // Otherwise, not at a spec boundary. Let's add names:
                lazy_static! {
                    static ref BEGINNING: Regex = Regex::new(r"^[A-Za-z ]*:\s*").unwrap();
                }
                let line = BEGINNING.replace_all(line, "");
                let line = line.trim_start();
                if entry.mnemonics.is_empty() {
                    entry.mnemonics = split_name_line(line);
                } else {
                    for element in split_name_line(line) {
                        if element.starts_with('#') {
                            entry.note = Some(element[2..element.len()].to_owned());
                        } else {
                            entry.synonyms.push(element);
                        }
                    }
                }
            }
        }
    }
    entries.push(entry);
    Ok(entries)
}

impl<'a> fmt::Display for ASCIIForDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let val = self.val.clone();
        write!(
            f,
            "Information{{value:{:?}, mnemonics:&{:?}, synonyms:&{:?}, note:{:?}}},",
            val.value, val.mnemonics, val.synonyms, val.note
        )?;
        Ok(())
    }
}

const PREAMBLE: &str = r#"/// Generated with `make names`

#[derive(Clone)]
pub struct Information {
    pub value: char,
    pub mnemonics: &'static [&'static str],
    pub synonyms: &'static [&'static str],
    pub note: Option<&'static str>,
}

#[rustfmt::skip]
"#;

pub fn write_ascii_name_data(output_dir: &Path, sorted_names: &mut fst_generator::Names) {
    fs::create_dir_all(output_dir).unwrap();
    let table = process_ascii_nametable().unwrap();

    for entry in table.clone() {
        sorted_names.insert(entry.mnemonics, entry.value);
        sorted_names.insert(entry.synonyms, entry.value);
    }

    let mut out = BufWriter::new(File::create(output_dir.join("names.rs")).unwrap());

    write!(&mut out, "{}", PREAMBLE).unwrap();
    writeln!(
        &mut out,
        "static PRINTABLE_CHARS: &[Information; {}] = &[",
        table.len()
    )
    .unwrap();
    for entry in table.clone() {
        writeln!(&mut out, "    {}", entry.for_display()).unwrap();
    }
    writeln!(&mut out, "];").unwrap();
}
