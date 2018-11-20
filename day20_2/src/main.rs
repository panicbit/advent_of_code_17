#![feature(option_filter)]

#[macro_use]
extern crate aoc;
extern crate itertools;

use std::ops::{Sub,AddAssign};
use std::iter::once;
use itertools::Itertools;

#[aoc(2017, 20, 2)]
fn main(input: &str) -> usize {
    let mut particles = input.lines().map(Particle::from_str).collect_vec();

    loop {
        let colliding_particles = particles.iter().enumerate().tuple_combinations()
        .filter_map(|((i,p1), (j,p2))| Some((i,j)).filter(|_| p1.position == p2.position))
        .flat_map(|(i, j)| once(i).chain(once(j)))
        .unique()
        .sorted_by(|i, j| j.cmp(i));

        for i in colliding_particles {
            particles.remove(i);
        }

        let next_particles = particles.iter().map(|p| p.simulate()).collect_vec();

        let can_collide = {
            let dists = particles
                .iter()
                .tuple_combinations()
                .map(|(p1, p2)| (p1.position - p2.position).dist());

            let next_dists = next_particles
                .iter()
                .tuple_combinations()
                .map(|(p1, p2)| (p1.position - p2.position).dist());

            dists.zip(next_dists).any(|(dist, next_dist)| dist > next_dist)
        };

        particles = next_particles;

        if !can_collide {
            break;
        }
    }

    particles.len()
}

#[derive(Debug, Copy, Clone)]
struct Particle {
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
}

impl Particle {
    fn from_str(str: &str) -> Self {
        let mut attrs = str.split(", ")
            .flat_map(|kv| kv.split("=").nth(1))
            .map(Vec3::from_str);

        Self {
            position: attrs.next().unwrap(),
            velocity: attrs.next().unwrap(),
            acceleration: attrs.next().unwrap(),
        }
    }

    fn simulate(mut self) -> Self {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Vec3 {
    fn dist(self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}


impl Sub for Vec3 {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self
    }
}

impl Vec3 {
    fn from_str(str: &str) -> Self {
        assert!(str.starts_with("<") && str.ends_with(">"));

        let mut it = str[1..str.len() - 1].split(",");

        Self {
            x: it.next().unwrap().parse().unwrap(),
            y: it.next().unwrap().parse().unwrap(),
            z: it.next().unwrap().parse().unwrap(),
        }
    }
}
