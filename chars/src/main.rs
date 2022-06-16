use std::env;

use chars::display;
use chars::human_names;

fn main() {
    let args = env::args().skip(1);
    for argument in args {
        let results = human_names::from_arg(argument.as_ref());
        if results.is_empty() {
            eprintln!("No results for “{}”.", argument);
        } else {
            for c in results {
                display::describe(c);
            }
        }
    }
}
