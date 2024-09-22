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

use anyhow::{Context, Result};
use regex::Regex;

use crate::fst_generator;

#[derive(Debug, Clone)]
struct AsciiEntry {
    value: char,
    mnemonics: Vec<String>,
    synonyms: Vec<String>,
    note: Option<String>,
}

#[derive(Debug)]
struct AsciiForDisplay<'a> {
    val: &'a AsciiEntry,
}

impl AsciiEntry {
    fn new(code: u8) -> AsciiEntry {
        AsciiEntry {
            value: code as char,
            mnemonics: vec![],
            synonyms: vec![],
            note: None,
        }
    }

    fn for_display(&self) -> AsciiForDisplay<'_> {
        AsciiForDisplay { val: self }
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

fn process_ascii_nametable() -> Result<Vec<AsciiEntry>, io::Error> {
    let mut char_code: u8 = 0;

    let mut entry = AsciiEntry::new(char_code);
    let mut entries: Vec<AsciiEntry> = vec![];

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
                    entry = AsciiEntry::new(char_code);
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

impl<'a> fmt::Display for AsciiForDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let val = self.val.clone();
        write!(
            f,
            "Information{{mnemonics:&{:?}, synonyms:&{:?}, note:{:?}}},",
            val.mnemonics, val.synonyms, val.note
        )?;
        Ok(())
    }
}

const PREAMBLE: &str = r#"/// Generated with `make names`

#[derive(Clone)]
pub struct Information {
    pub mnemonics: &'static [&'static str],
    pub synonyms: &'static [&'static str],
    pub note: Option<&'static str>,
}

#[rustfmt::skip]
"#;

pub fn write_ascii_name_data(
    output_dir: &Path,
    sorted_names: &mut fst_generator::Names,
) -> Result<()> {
    fs::create_dir_all(output_dir)?;
    let table = process_ascii_nametable()?;

    for entry in table.clone() {
        sorted_names.insert(entry.mnemonics, entry.value);
        sorted_names.insert(entry.synonyms, entry.value);
    }

    let mut out = BufWriter::new(
        File::create(output_dir.join("names.rs")).context("Creading ASCII names.rs")?,
    );

    write!(&mut out, "{}", PREAMBLE)?;
    writeln!(
        &mut out,
        "static PRINTABLE_CHARS: &[Information; {}] = &[",
        table.len()
    )?;
    for entry in table {
        writeln!(&mut out, "    {}", entry.for_display())?;
    }
    writeln!(&mut out, "];")?;
    Ok(())
}
