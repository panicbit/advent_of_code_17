#[macro_use]
extern crate aoc;

aoc!(2017, 13, 1, |input| {
    let mut layers = input.lines().map(|line| {
        let mut line = line.trim().split(": ");
        Layer {
            depth: line.next().unwrap().parse().unwrap(),
            range: line.next().unwrap().parse().unwrap(),
            scanner_pos: 0,
            scanner_dir: 1,
        }
    }).collect::<Vec<_>>();
    let mut severity = 0;
    let mut pos = 0;

    for i in 0..layers.len() {
        while pos <= layers[i].depth {
            {
                let layer = &layers[i];
                if pos == layer.depth && layer.scanner_pos == 0 {
                    severity += layer.depth * layer.range;
                }
            }

            advance_scanners(&mut layers);

            pos += 1;
        }
    }

    severity
});

fn advance_scanners(layers: &mut Vec<Layer>) {
    for layer in layers {
        if (layer.scanner_dir < 0 && layer.scanner_pos == 0)
        || (layer.scanner_dir > 0 && layer.scanner_pos + 1 == layer.range) {
            layer.scanner_dir *= -1;
        }

        layer.scanner_pos += layer.scanner_dir;
    }
}

#[derive(Debug)]
struct Layer {
    depth: isize,
    range: isize,
    scanner_pos: isize,
    scanner_dir: isize,
}
