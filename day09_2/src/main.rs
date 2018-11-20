#[macro_use]
extern crate aoc;

use self::Mode::*;

#[aoc(2017, 09, 2)]
fn main(input: &str) -> i32 {
    let mut garbage = 0;
    let mut chars = input.chars();
    let mut mode = Normal;

    while let Some(ch) = chars.next() {
        match (mode, ch) {
            (Normal, '<') => mode = Garbage,
            (Garbage, '!') => { chars.next(); },
            (Garbage, '>') => mode = Normal,
            (Garbage, _) => garbage += 1,
            _ => {},
        }
    }

    garbage
}

#[derive(Copy,Clone)]
enum Mode {
    Normal,
    Garbage,
}
