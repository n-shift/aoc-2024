use std::fs::read_to_string;

const INPUT_FILE: &str = "./input/8.txt";

const EMPTY: char = '.';
const ANTINODE: char = '#';

#[derive(Clone)]
struct Map {
    data: Vec<Vec<char>>,
    antinodes: Vec<Vec<char>>,
    antennas: Vec<char>,
    height: usize,
    width: usize,
}

impl Map {
    fn from_data(data: Vec<Vec<char>>) -> Map {
        let height = data.len();
        let width = data[0].len();
        let antinodes = vec![vec![EMPTY; width]; height];
        let mut antennas: Vec<char> = Vec::new();
        for c in data.iter().flatten().filter(|&x| *x != EMPTY) {
            if !antennas.contains(c) {
                antennas.push(*c)
            }
        }
        Map {
            data,
            antinodes,
            antennas,
            height,
            width,
        }
    }

    fn compute_antinodes(&mut self) {
        for antenna in &self.antennas {
            let antennas = &self.get_antenna_positions(*antenna);
            for i in 0..antennas.len() - 1 {
                for j in i + 1..antennas.len() {
                    let (ax, ay) = (antennas[i].0 as isize, antennas[i].1 as isize);
                    let (bx, by) = (antennas[j].0 as isize, antennas[j].1 as isize);
                    let (dx, dy) = (bx - ax, by - ay);
                    let (an_x, an_y) = (ax - dx, ay - dy);
                    if self.contains(an_x, an_y) {
                        self.antinodes[an_y as usize][an_x as usize] = ANTINODE;
                    }
                    let (an_x, an_y) = (bx + dx, by + dy);
                    if self.contains(an_x, an_y) {
                        self.antinodes[an_y as usize][an_x as usize] = ANTINODE;
                    }
                }
            }
        }
    }

    fn compute_antinodes_with_resonant_frequencies(&mut self) {
        for antenna in &self.antennas {
            let antennas = &self.get_antenna_positions(*antenna);
            for i in 0..antennas.len() - 1 {
                for j in i + 1..antennas.len() {
                    let (ax, ay) = (antennas[i].0 as isize, antennas[i].1 as isize);
                    let (bx, by) = (antennas[j].0 as isize, antennas[j].1 as isize);
                    let (dx, dy) = (bx - ax, by - ay);

                    let mut k = 0;
                    while self.contains(ax - (k * dx), ay - (k * dy)) {
                        let (an_x, an_y) = (ax - (k * dx), ay - (k * dy));
                        self.antinodes[an_y as usize][an_x as usize] = ANTINODE;
                        k += 1;
                    }
                    k = 0;
                    while self.contains(bx + (k * dx), by + (k * dy)) {
                        let (an_x, an_y) = (bx + (k * dx), by + (k * dy));
                        self.antinodes[an_y as usize][an_x as usize] = ANTINODE;
                        k += 1;
                    }
                }
            }
        }
    }

    fn contains(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize
    }

    fn get_antenna_positions(&self, antenna: char) -> Vec<(usize, usize)> {
        let mut positions: Vec<(usize, usize)> = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.data[y][x] == antenna {
                    positions.push((x, y));
                }
            }
        }
        positions
    }

    fn count_antinodes(&self) -> usize {
        self.antinodes
            .iter()
            .flatten()
            .filter(|&x| *x == ANTINODE)
            .count()
    }
}

fn part_one(mut map: Map) {
    map.compute_antinodes();
    println!("Total antinodes: {}", map.count_antinodes());
}

fn part_two(mut map: Map) {
    map.compute_antinodes_with_resonant_frequencies();
    println!("Total antinodes with resonance: {}", map.count_antinodes());
}

fn main() {
    let contents = read_to_string(INPUT_FILE).unwrap();
    let map_data = contents
        .trim()
        .lines()
        .map(|x| x.chars().collect())
        .collect();
    let map = Map::from_data(map_data);

    part_one(map.clone());
    part_two(map);
}
