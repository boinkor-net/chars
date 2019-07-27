//! Build script for chars.
//!
//! This just runs the generator/ subproject, to generate the source files from files in data/.

use std::env;
use std::error::Error;
use std::path::Path;

extern crate chars_data;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    chars_data::generate_files(Path::new(&out_dir))?;
    Ok(())
}
