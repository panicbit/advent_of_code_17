#[macro_use]
extern crate aoc;

#[aoc(2017, 20, 1)]
fn main(input: &str) -> usize {
    let particles = input.lines().map(Particle::from_str).collect::<Vec<_>>();

    particles
        .iter()
        .enumerate()
        .min_by_key(|&(_, p)| p.acceleration.dist())
        .map(|(i, _)| i)
        .unwrap()
}

#[derive(Debug)]
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
