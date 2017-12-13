#[macro_use]
extern crate aoc;

aoc!(2017, 13, 1, |input| {
    let mut layers = input.lines().map(Layer::from_str).collect::<Vec<_>>();
    let mut severity = 0;
    let mut pos = 0;

    for i in 0..layers.len() {
        while pos <= layers[i].depth() {
            if layers[i].has_caught(pos) {
                severity += layers[i].severity();
            }

            for layer in &mut layers {
                layer.advance_scanner();
            }

            pos += 1;
        }
    }

    severity
});

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

    fn advance_scanner(&mut self) {
        self.scanner_step += 1;
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

    fn severity(&self) -> isize {
        self.depth * self.range
    }

    fn has_caught(&self, pos: isize) -> bool {
        pos == self.depth && self.scanner_pos() == 0
    }

    fn depth(&self) -> isize {
        self.depth
    }
}
