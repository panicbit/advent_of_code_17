#[macro_use]
extern crate aoc;

use self::Mode::*;

aoc!(2017, 09, 1, |input| {
    let mut score = 0;
    let mut level = 0;
    let mut chars = input.chars();
    let mut mode = Normal;

    while let Some(ch) = chars.next() {
        match (mode, ch) {
            (Normal, '{') => level += 1,
            (Normal, '}') => {
                score += level;
                level -= 1;
            },
            (Normal, '<') => mode = Garbage,
            (Garbage, '!') => { chars.next(); },
            (Garbage, '>') => mode = Normal,
            _ => {},
        }
    }

    score
});

#[derive(Copy,Clone)]
enum Mode {
    Normal,
    Garbage,
}
