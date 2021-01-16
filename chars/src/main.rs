use std::env;

use chars::display;
use chars::human_names;

fn main() {
    let args = env::args()
        .skip(1)
        .flat_map(|argument| human_names::from_arg(argument.as_ref()));
    for c in args {
        display::describe(c);
    }
}
