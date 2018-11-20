#[macro_use]
extern crate aoc;

use std::str::FromStr;
use std::collections::HashMap;
use std::iter::once;

#[aoc(2017, 19, 2)]
fn main(input: &str) -> usize {
    let maze = input.parse::<Maze>().unwrap();

    MazeIter::new(&maze).count()
}

struct MazeIter<'a> {
    maze: &'a Maze,
    pos: (isize, isize),
    dir: Dir,
}

impl<'a> MazeIter<'a> {
    fn new(maze: &'a Maze) -> Self {
        let (x, y) = maze.find_beginning();
        MazeIter {
            maze,
            pos: (x, y - 1),
            dir: Dir::Down,
        }
    }
}

impl<'a> Iterator for MazeIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let turn_dirs = self.dir.turn_dirs();
        let res = once(self.dir)
            .chain(turn_dirs.iter().cloned())
            .map(|dir| {
                let pos = dir.step(self.pos);
                (pos, dir, self.maze.get(pos))
            })
            .find(|&(_, _, ch)| ch != ' ');

        if let Some((pos, dir, ch)) = res {
            self.pos = pos;
            self.dir = dir;
            return Some(ch);
        }

        None
    }
}

#[derive(Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn_dirs(self) -> [Dir; 2] {
        match self {
            Dir::Up | Dir::Down => [Dir::Left, Dir::Right],
            Dir::Left | Dir::Right => [Dir::Up, Dir::Down],
        }
    }

    fn rel_pos(self) -> (isize, isize) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }

    fn step(self, (x, y): (isize, isize)) -> (isize, isize) {
        let (rx, ry) = self.rel_pos();
        (x + rx, y + ry)
    }
}

struct Maze(HashMap<(isize, isize), char>);

impl Maze {
    fn get(&self, pos: (isize, isize)) -> char {
        self.0.get(&pos).cloned().unwrap_or(' ')
    }

    fn find_beginning(&self) -> (isize, isize) {
        self.0
            .iter()
            .find(|&(&(_, y), &ch)| y == 0 && ch == '|')
            .map(|(&pos, _)| pos)
            .unwrap()
    }
}

impl FromStr for Maze {
    type Err = ();

    fn from_str(maze: &str) -> Result<Self, Self::Err> {
        let maze = maze.lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, ch)| ch != ' ')
                    .map(move |(x, ch)| ((x as isize, y as isize), ch))
            })
            .collect();

        Ok(Maze(maze))
    }
}
