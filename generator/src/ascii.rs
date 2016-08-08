///# Parse ascii(1) `nametable` files
///
/// Nametables have an almost deceptively simple format:
///
///```
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
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};
use std::io::{BufRead, Write};
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
struct ASCIIForDisplay<'a> { val: &'a ASCIIEntry }

#[derive(Debug)]
struct  ASCIIForReading<'a> { val: &'a ASCIIEntry }

impl ASCIIEntry {
    fn new(code: u8) -> ASCIIEntry {
        ASCIIEntry{
            value: code as char,
            mnemonics: vec!(),
            synonyms: vec!(),
            note: None,
        }
    }

    fn for_display(&self) -> ASCIIForDisplay {
        ASCIIForDisplay {val: self }
    }

    fn for_reading(&self) -> ASCIIForReading {
        ASCIIForReading {val: self }
    }
}

fn split_name_line(line: &str) -> Vec<String> {
    lazy_static! {
        static ref QUOTES: Regex = Regex::new("\",\\s*\"").unwrap();
        static ref RIGHT_QUOTE: Regex = Regex::new("\",\\s*$").unwrap();
        static ref LEFT_QUOTE: Regex = Regex::new("^\"").unwrap();
    }
    let line = line.trim_left();
    let line = LEFT_QUOTE.replace_all(RIGHT_QUOTE.replace_all(line, "$1").as_str(), "");
    QUOTES.split(line.as_str()).map(|s| s.to_owned()).collect()
}

fn process_ascii_nametable(input: File) -> Result<Vec<ASCIIEntry>, io::Error> {
    let mut char_code: u8 = 0;

    let mut entry = ASCIIEntry::new(char_code);
    let mut entries: Vec<ASCIIEntry> = vec!();

    let reader = BufReader::new(&input);
    for line in reader.lines() {
        let line = try!(line);
        let line = line.as_str();
        match line.chars().next() {
            Some('#') | None => {},
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
                let line = line.trim_left();
                if entry.mnemonics.len() == 0 {
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
        try!(write!(f, "Information{{value:{:?}, mnemonics:&{:?}, synonyms:&{:?}, note:{:?}}},",
                    val.value, val.mnemonics, val.synonyms, val.note));
        Ok(())
    }
}

impl<'a> fmt::Display for ASCIIForReading<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let val = self.val.clone();
        for mnemo in val.mnemonics {
            if mnemo.len() != 1 {
                try!(write!(f, "({:?}, {:?}),", mnemo.to_lowercase(), val.value));
            }
        }
        for syn in val.synonyms {
            try!(write!(f, "({:?}, {:?}),", syn.to_lowercase(), val.value));
            // Allow searching for "equals" in addition to "equals sign":
            if syn.to_lowercase().ends_with("sign") {
                try!(write!(f, "({:?}, {:?}),", syn[0..syn.len()-5].to_lowercase(), val.value));
            }

            // Allow searching for "seven" in addition to "digit seven":
            if syn.to_lowercase().starts_with("digit") {
                try!(write!(f, "({:?}, {:?}),", syn[6..syn.len()].to_lowercase(), val.value));
            }
        }
        Ok(())
    }
}

const PREAMBLE: &'static str = r#"
#[derive(Clone)]
pub struct Information {
    pub value: char,
    pub mnemonics: &'static [&'static str],
    pub synonyms: &'static [&'static str],
    pub note: Option<&'static str>,
}
"#;

pub fn write_ascii_name_data(nametable: &Path, output: &Path, sorted_names: &mut fst_generator::Names) {
    let table = process_ascii_nametable(File::open(nametable).unwrap()).unwrap();

    for entry in table.clone() {
        sorted_names.insert(entry.mnemonics, entry.value);
        sorted_names.insert(entry.synonyms, entry.value);
    }

    let mut out = BufWriter::new(File::create(output).unwrap());

    write!(&mut out, "/// Generated with `make names`\n").unwrap();
    write!(&mut out, "{}\n\n", PREAMBLE).unwrap();
    write!(&mut out, "pub static PRINTABLE_CHARS: &'static [Information; {}] = &[\n", table.len()).unwrap();
    for entry in table.clone() {
        write!(&mut out, "    {}\n", entry.for_display()).unwrap();
    }
    write!(&mut out, "];\n\n\n").unwrap();

    write!(&mut out, "pub static NAMES: &'static [(&'static str, char)] = &[\n").unwrap();
    for entry in table.clone() {
        write!(&mut out, "    {}\n", entry.for_reading()).unwrap();

    }
    write!(&mut out, "];\n").unwrap();
}
