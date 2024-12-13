use std::collections::HashMap;/*
const INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";*/
const INPUT: &str = include_str!("../../input/8.txt");
const BOUNDS: (isize, isize) = (49, 49);

#[derive(Clone)]
struct Frequencies {
    used: Vec<(isize, isize)>,
    map: HashMap<char, Vec<(usize, usize)>>,
}

impl Frequencies {
    fn init(input: &str) -> Self {
        let mut used: Vec<(isize, isize)> = Vec::new();
        let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        input.split('\n').enumerate().for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, c)| {
                if c != '.' {
                    used.push((x as isize, y as isize));
                    match map.get_mut(&c) {
                        Some(v) => v.push((x, y)),
                        None => _ = map.insert(c, vec![(x, y)]),
                    };
                }
            });
        });
        Self { used, map }
    }
    fn invalid(&mut self, node: (isize, isize)) -> bool {
        let res = node.0 < 0 || node.1 < 0 || node.0 > BOUNDS.0 || node.1 > BOUNDS.1 || self.used.contains(&node);
        self.used.push(node);
        res
    }
}

fn anti_node(node_1: (usize, usize), node_2: (usize, usize)) -> (isize, isize) {
    let node_1 = (node_1.0 as isize, node_1.1 as isize);
    let node_2 = (node_2.0 as isize, node_2.1 as isize);
    (
        node_1.0 - node_2.0 + node_1.0,
        node_1.1 - node_2.1 + node_1.1,
    )
}

fn main() {
    let freqs = Frequencies::init(INPUT);
    let mut freqs_check = freqs.clone();
    let nodes = freqs.map.values().map(|v| {
        (0..v.len()).map(|a| {
            (0..v.len()).filter(|b| {
                if a != *b {
                    !freqs_check.invalid(anti_node(v[a], v[*b]))
                } else { false }
            }).count()
        }).sum::<usize>()
    }).sum::<usize>();
    println!("{nodes}");
}
