#[macro_use] extern crate aoc;

use self::Direction::*;

#[aoc(2017, 3, 1)]
fn main(input: &str) -> i32 {
    let input = input.parse::<i32>().unwrap();
    let mut dir = Direction::E;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut step = 1;
    let mut amount = 1;
    let mut inc_amount = false;
    
    loop {
        for _ in 0..amount {

            match dir {
                N => y += 1,
                W => x -= 1,
                S => y -= 1,
                E => x += 1,
            }

            step += 1;

            if step >= input {
                return x.abs() + y.abs();
            }
        }


        inc_amount = match inc_amount {
            false => true,
            true => {
                amount += 1;
                false
            }
        };

        dir.rotate();
    }    
}

enum Direction {
    N, E, S, W
}

impl Direction {
    fn rotate(&mut self) {
        *self = match *self {
            N => W,
            W => S,
            S => E,
            E => N,
        }
    }
}
