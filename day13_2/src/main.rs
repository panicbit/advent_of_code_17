#[macro_use]
extern crate aoc;

#[aoc(2017, 13, 2)]
fn main(input: &str) -> isize {
    let layers = input.lines().map(Layer::from_str).collect::<Vec<_>>();

    let mut delay = 0;
    while gets_caught(&layers, delay) {
        delay += 1;
    }

    delay
}

fn gets_caught(layers: &Vec<Layer>, delay: isize) -> bool {
    for layer in layers {
        let step = layer.depth() + delay;

        if layer.scanner_at_top(step) {
            return true;
        }
    }

    false
}

#[derive(Debug,Clone)]
struct Layer {
    depth: isize,
    range: isize,
}

impl Layer {
    fn from_str(line: &str) -> Self {
        let mut line = line.trim().split(": ");
        let depth = line.next().unwrap().parse().unwrap();
        let range = line.next().unwrap().parse().unwrap();

        Layer {
            depth,
            range,
        }
    }

    fn scanner_at_top(&self, step: isize) -> bool {
        let range = self.range;
        let cycle = if range == 1 { 1 } else { 2 * (range-1) };
        step % cycle == 0
    }

    fn depth(&self) -> isize {
        self.depth
    }
}
