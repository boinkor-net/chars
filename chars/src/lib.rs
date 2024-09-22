#![deny(clippy::all)] // And make every default clippy warning fatal.

#[macro_use]
extern crate lazy_static;

mod ascii;
mod unicode;

pub mod display;
pub mod human_names;
