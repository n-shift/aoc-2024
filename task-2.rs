use std::fs::File;
use std::io::{prelude::*, BufReader};

fn is_safe(numerals: &[u8]) -> bool {
    let increasing = numerals[0] < numerals[1];
    numerals[..].windows(2).all(|w| {
        w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3 && (w[0] < w[1]) == increasing
    })
}

fn main() {
    let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(file);
    let mut safety_counter = 0;
    let mut damp_counter = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let numerals = line
            .split_whitespace()
            .map(|n| n.parse::<u8>().unwrap())
            .collect::<Vec<_>>();

        if is_safe(&numerals[..]) {
            safety_counter += 1;
        }
        if (0..numerals.len()).any(|i| is_safe(&[&numerals[..i], &numerals[i + 1..]].concat())) {
            damp_counter += 1;
        }
    }
    println!("Default: {safety_counter}");
    println!("Dampened: {damp_counter}");
}
