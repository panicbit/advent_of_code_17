#[macro_use]
extern crate aoc;

use std::collections::{HashMap,HashSet};

aoc!(2017, 12, 2, |input| {
    let pipes = input.lines().map(|line| {
        let mut line = line.trim().split(" <-> ");
        let first = parse_int(line.next().unwrap());
        let others = line.next().unwrap()
            .split(", ")
            .map(parse_int)
            .collect::<Vec<_>>();
        (first, others)
    }).collect::<HashMap<_,_>>();

    let mut num_groups = 0;
    let mut visited = HashSet::new();
    
    for &program in pipes.keys() {
        if visited.contains(&program) {
            continue
        }
        visit_group(program, &pipes, &mut visited);
        num_groups += 1;
    }

    num_groups
});

fn visit_group(program: i32, pipes: &HashMap<i32, Vec<i32>>, visited: &mut HashSet<i32>) {
    if visited.contains(&program) {
        return;
    }

    visited.insert(program);

    for &program in &pipes[&program] {
        visit_group(program, pipes, visited);
    }
}

fn parse_int(n: &str) -> i32 {
    n.parse().unwrap()
}
