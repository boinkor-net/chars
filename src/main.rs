extern crate unicode_names;

use std::env;

fn main() {
    for argument in env::args().skip(1) {
        match unicode_names::character(argument.as_ref()) {
            Some(c) =>  println!("{} = {}", c, argument),
            None => println!("Not found"),
        }
    }
}
