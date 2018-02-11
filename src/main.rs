#![cfg_attr(feature = "clippy", feature(plugin))] // Use clippy if it's available
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", deny(clippy))] // And make every warning fatal.

extern crate byteorder;
extern crate fst;
#[macro_use]
extern crate lazy_static;
extern crate unicode_names;
extern crate unicode_width;

use std::env;

mod display;
mod human_names;
mod ascii;
mod unicode;

fn main() {
    let args = env::args()
        .skip(1)
        .flat_map(|argument| human_names::from_arg(argument.as_ref()));
    for c in args {
        display::describe(c);
    }
}
