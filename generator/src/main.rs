use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "print this help");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") || matches.free.len() != 2 {
        println!(
            "{}",
            opts.usage(&format!(
                "USAGE: {} [options] data-dir output-src-dir\n -- generate character name table",
                program
            ))
        );
        return;
    }
    let data_dirname = matches.free[0].clone();
    let data_dir = Path::new(data_dirname.as_str());

    let src_dirname = matches.free[1].clone();
    let src_dir = Path::new(src_dirname.as_str());

    generator::generate_files(data_dir, src_dir).unwrap();
}
