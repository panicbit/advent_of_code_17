#[macro_use]
extern crate aoc;

aoc!(2017, 13, 1, |input| {
    let layers = input.lines().map(Layer::from_str).collect::<Vec<_>>();
    let mut severity = 0;

    for layer in layers {
        let step = layer.depth();

        if layer.scanner_at_top(step) {
            severity += layer.severity();
        }
    }

    severity
});

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

    fn severity(&self) -> isize {
        self.depth * self.range
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
