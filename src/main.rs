extern crate unicode_names;
extern crate byteorder;
extern crate fst;
#[macro_use] extern crate lazy_static;

use std::env;

mod display;
mod human_names;
mod ascii;
mod unicode;

fn main() {
    let args =
        env::args().skip(1)
        .flat_map(|argument| human_names::from_arg(argument.as_ref()));
    for c in args {
        display::describe(c);
    }
}
