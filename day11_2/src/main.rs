#[macro_use]
extern crate aoc;

use std::cmp::max;

#[aoc(2017, 11, 2)]
fn main(input: &str) -> i32 {
    let steps = input.trim().split(',').map(Dir::from_str);
    let mut pos = HexPos::default();
    let mut max_dist = 0;

    for step in steps {
        pos.step(step);
        max_dist = max(max_dist, max(pos.n.abs(), pos.se.abs()));
    }
    
    max_dist
}

#[derive(Default,Copy,Clone)]
struct HexPos {
    n: i32,
    se: i32,
}

impl HexPos {
    fn step(&mut self, dir: Dir) {
        match dir {
            Dir::N => self.n += 1,
            Dir::S => self.n -= 1,
            Dir::SE => self.se += 1,
            Dir::NW => self.se -= 1,
            Dir::NE => {
                self.n += 1;
                self.se += 1;
            },
            Dir::SW => {
                self.n -= 1;
                self.se -= 1;
            },
        }
    }
} 

enum Dir {
    N, S,
    NW, SE,
    SW, NE,
}

impl Dir {
    fn from_str(dir: &str) -> Dir {
        match dir {
            "n" => Dir::N,
            "s" => Dir::S,
            "nw" => Dir::NW,
            "se" => Dir::SE,
            "sw" => Dir::SW,
            "ne" => Dir::NE,
            other => panic!("Unknown dir {:?}", other),
        }
    }
}
