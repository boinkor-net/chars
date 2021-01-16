#![cfg_attr(feature = "clippy", feature(plugin))] // Use clippy if it's available
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", deny(clippy))] // And make every warning fatal.

#[macro_use]
extern crate lazy_static;

mod ascii;
mod unicode;

pub mod display;
pub mod human_names;
