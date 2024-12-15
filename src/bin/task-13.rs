const INPUT: &str = include_str!("../../input/13.txt");
struct ClawMachine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

fn parse_line(line: &str, correct: bool) -> (usize, usize) {
    // into map
    let line = line.split(' ').rev().take(2).collect::<Vec<_>>();
    let x = if correct { 10000000000000 } else { 0 }
        + line[1][2..line[1].len() - 1].parse::<usize>().unwrap();
    let y = if correct { 10000000000000 } else { 0 }
        + line[0][2..line[0].len()].parse::<usize>().unwrap();
    (x, y)
}

impl ClawMachine {
    fn init(note: &str, correct: bool) -> Self {
        let parsed = note
            .lines()
            .enumerate()
            .map(|(i, l)| {
                if i == 2 {
                    parse_line(l, correct)
                } else {
                    parse_line(l, false)
                }
            })
            .collect::<Vec<_>>();
        Self {
            a: parsed[0],
            b: parsed[1],
            prize: parsed[2],
        }
    }
    fn solve(&self) -> (usize, usize) {
        let det = (self.a.0 * self.b.1) as isize - (self.a.1 * self.b.0) as isize;
        let det_a = (self.prize.0 * self.b.1) as isize - (self.prize.1 * self.b.0) as isize;
        let det_b = (self.a.0 * self.prize.1) as isize - (self.a.1 * self.prize.0) as isize;
        let a: usize = (det_a / det).try_into().unwrap_or(0);
        let b: usize = (det_b / det).try_into().unwrap_or(0);
        (a, b)
    }
    fn cost(&self) -> usize {
        let (a, b) = self.solve();
        if a * self.a.0 + b * self.b.0 == self.prize.0
            && a * self.a.1 + b * self.b.1 == self.prize.1
        {
            3 * a + b
        } else {
            0
        }
    }
}

fn main() {
    let total = INPUT
        .split("\n\n")
        .map(|n| ClawMachine::init(n, false).cost())
        .sum::<usize>();
    println!("Std: {total}");
    let total = INPUT
        .split("\n\n")
        .map(|n| ClawMachine::init(n, true).cost())
        .sum::<usize>();
    println!("Cor: {total}");
}
