#![cfg_attr(feature = "clippy", feature(plugin))] // Use clippy if it's available
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", deny(clippy))] // And make every warning fatal.

extern crate byteorder;
extern crate fst;
#[macro_use]
extern crate lazy_static;
extern crate unicode_names2;
extern crate unicode_width;

mod ascii;
mod unicode;

pub mod display;
pub mod human_names;
