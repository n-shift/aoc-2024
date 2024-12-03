use std::fs::File;
use std::io::{prelude::*, BufReader};
fn main() {
    let file = File::open("input-1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut l1: [u32; 1000] = [0; 1000];
    let mut l2: [u32; 1000] = [0; 1000];
    for (idx, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let numerals: Vec<&str> = line.split_whitespace().collect();
        l1[idx] = numerals[0].parse::<u32>().unwrap();
        l2[idx] = numerals[1].parse::<u32>().unwrap();
    }
    l1.sort();
    l2.sort();
    let mut distance: u32 = 0;
    let mut similarity: u32 = 0;
    for index in 0..1000 {
        distance += l1[index].abs_diff(l2[index]);
        similarity += l1[index] * (l2.iter().filter(|&n| *n == l1[index]).count() as u32)
    }
    println!("Distance: {distance}");
    println!("Similarity: {similarity}");
}
