// this whole file is just some blabbering, not even sure i've managed to optimize it
const fn to_u32(bytes: &[u8], start: usize, end: usize) -> u32 {
    let mut res: u32 = 0;
    let mut idx = start;
    while idx < end {
        res = 10 * res + (bytes[idx] - b'0') as u32;
        idx += 1;
    }
    res
}
// short for split & parse
const fn sp(bytes: &[u8]) -> [[u32; 1000]; 2] {
    let mut res = [[0; 1000]; 2];
    let mut select_res = 0;
    let mut idx_start = 0;
    let mut idx_curr = 0;
    let mut idx = 0;
    while idx < 1000 {
        while idx_curr < bytes.len() && bytes[idx_curr] != b' ' && bytes[idx_curr] != b'\n' {
            idx_curr += 1;
        }
        // i want to sleep
        if to_u32(bytes, idx_start, idx_curr) != 0 {
            res[select_res][idx] = to_u32(bytes, idx_start, idx_curr);
            if select_res == 0 {
                select_res = 1;
            } else {
                select_res = 0;
                idx += 1;
            }
        }
        idx_curr += 1;
        idx_start = idx_curr;
    }
    res
}

const FILE: &[u8] = include_bytes!("input-1.txt");

fn main() {
    let mut list = sp(FILE);
    list[0].sort();
    list[1].sort();
    let distance: u32 = list[0]
        .iter()
        .zip(list[1].iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();
    let similarity: u32 = list[0]
        .iter()
        .map(|n| n * list[1].iter().filter(|m| *m == n).count() as u32)
        .sum();
    println!("Distance: {distance}");
    println!("Similarity: {similarity}");
}
