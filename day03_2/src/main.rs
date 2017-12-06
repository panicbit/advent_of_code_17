use std::collections::HashMap;
use self::Direction::*;

fn main() {
    let input = 325489;
    let mut dir = Direction::E;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut amount = 1;
    let mut inc_amount = false;
    let mut values = HashMap::new();
    
    loop {
        for _ in 0..amount {
            let mut value =
                  values.get(&(x-1, y-1)).unwrap_or(&0)
                + values.get(&(x-1, y  )).unwrap_or(&0)
                + values.get(&(x-1, y+1)).unwrap_or(&0)
                + values.get(&(x  , y-1)).unwrap_or(&0)
                + values.get(&(x  , y+1)).unwrap_or(&0)
                + values.get(&(x+1, y-1)).unwrap_or(&0)
                + values.get(&(x+1, y  )).unwrap_or(&0)
                + values.get(&(x+1, y+1)).unwrap_or(&0);

            if value == 0 {
                value = 1;
            }

            values.insert((x, y), value);

            if value > input {
                println!("{}", values[&(x, y)]); 
                return;
            }

            match dir {
                N => y += 1,
                W => x -= 1,
                S => y -= 1,
                E => x += 1,
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
