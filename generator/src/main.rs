extern crate getopts;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::fmt;

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

fn process_ascii_nametable(input: File) -> Result<Vec<ASCIIEntry>, std::io::Error> {
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
                try!(write!(f, "({:?}, {:?}),", mnemo, val.value));
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "print this help");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m, Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") || matches.free.is_empty() {
        println!("{}", opts.usage(&format!("USAGE: {} [options] input-file\n -- generate character name table",
                                          program)));
        return
    }
    let input = matches.free[0].clone();
    let table = process_ascii_nametable(File::open(input).unwrap()).unwrap();
    println!("/// Generated with {:?}", args.clone());
    println!("{}\n", PREAMBLE);
    println!("pub static PRINTABLE_CHARS: &'static [Information; {}] = &[", table.len());
    for entry in table.clone() {
        println!("    {}", entry.for_display());
    }
    println!("];\n\n");

    println!("pub static NAMES: &'static [(&'static str, char)] = &[");
    for entry in table.clone() {
        println!("    {}", entry.for_reading());
    }
    println!("];");
}
