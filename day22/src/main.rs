#[macro_use]
extern crate aoc;

use std::collections::HashSet;

#[aoc(2017, 22, 1)]
fn main(input: &str) -> usize {
    let width = input.lines().next().map(str::len).unwrap_or(0) as isize;
    let height = input.lines().count() as isize;

    let mut grid: HashSet<(isize, isize)> = input.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line
            .chars()
            .enumerate()
            .filter(|&(_, ch)| ch == '#')
            .map(move |(x, _)| (x as isize, y as isize))
        })
        .collect();
    
    let mut carrier = Carrier::new(width / 2, height / 2);
    let n_bursts = 10000;

    for _ in 0..n_bursts {
        carrier.do_your_thing(&mut grid);
    }

    carrier.infected
}

struct Carrier {
    x: isize,
    y: isize,
    dir: Dir,
    infected: usize,
}

impl Carrier {
    fn new(x: isize, y: isize) -> Self {
        Carrier {
            x,
            y,
            dir: Dir::Up,
            infected: 0,
        }
    }

    fn do_your_thing(&mut self, grid: &mut HashSet<(isize, isize)>) {
        let pos = self.pos();
        let is_current_node_infected = grid.contains(&pos);

        if is_current_node_infected {
            self.turn_right();
            grid.remove(&pos);
        } else {
            self.turn_left();
            grid.insert(pos);
            self.infected += 1;
        }

        self.move_forward();
    }

    fn move_forward(&mut self) {
        match self.dir {
            Dir::Up => self.y -= 1,
            Dir::Left => self.x -= 1,
            Dir::Down => self.y += 1,
            Dir::Right => self.x += 1,
        }
    }

    fn turn_left(&mut self) {
        self.dir = match self.dir {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
        }
    }

    fn turn_right(&mut self) {
        self.dir = match self.dir {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    fn pos(&self) -> (isize, isize) {
        (self.x, self.y)
    }
}

enum Dir {
    Up, Down, Left, Right,
}
