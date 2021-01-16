//! Generator for chars(1) data files.

#[macro_use]
extern crate lazy_static;

use anyhow::{Context, Result};
use std::path::Path;

mod ascii;
mod fst_generator;
mod unicode;

/// Runs the code generator and writes files.
pub fn generate_files(src_dir: &Path) -> Result<()> {
    let mut sorted_names = fst_generator::Names::new();

    ascii::write_ascii_name_data(&src_dir.join("ascii"), &mut sorted_names)
        .context("Processing ASCII name data")?;

    unicode::read_names(&mut sorted_names, unicode::name_aliases())
        .context("Reading unicode name aliases")?;
    unicode::read_names(&mut sorted_names, unicode::unicode_data())
        .context("Reading unicode data")?;
    unicode::write_name_data(&sorted_names, &src_dir.join("unicode/"))
        .context("Writing unicode name data")?;
    Ok(())
}
