#[macro_use]
extern crate aoc;

aoc!(2017, 13, 2, |input| {
    let mut layers = input.lines().map(Layer::from_str).collect::<Vec<_>>();

    let mut delay = 0;
    while gets_caught(&mut layers, delay) {
        delay += 1;
    }

    delay
});

fn gets_caught(layers: &mut Vec<Layer>, delay: isize) -> bool {
    for layer in layers {
        let step = layer.depth() + delay;
        layer.set_scanner_step(step);

        if layer.scanner_pos() == 0 {
            return true;
        }
    }

    false
}

#[derive(Debug,Clone)]
struct Layer {
    depth: isize,
    range: isize,
    scanner_step: isize,
}

impl Layer {
    fn from_str(line: &str) -> Self {
        let mut line = line.trim().split(": ");
        let depth = line.next().unwrap().parse().unwrap();
        let range = line.next().unwrap().parse().unwrap();

        Layer {
            depth,
            range,
            scanner_step: 0,
        }
    }

    fn scanner_pos(&self) -> isize {
        let range = self.range;
        let cycle = if range == 1 { 1 } else { 2 * (range-1) };
        let step = self.scanner_step % cycle;
        let modstep = step % range;
        let if_down = step / range % 2;
        let if_up = 1 - if_down;

        if_up * step + if_down * (range - modstep - 2)
    }

    fn set_scanner_step(&mut self, step: isize) {
        self.scanner_step = step;
    }

    fn depth(&self) -> isize {
        self.depth
    }
}
