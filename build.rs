//! Build script for chars.
//!
//! This just runs the generator/ subproject, to generate the source files from files in data/.

use std::env;
use std::error::Error;
use std::path::Path;

extern crate generator;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    generator::generate_files(Path::new("./data"), Path::new(&out_dir))?;
    Ok(())
}