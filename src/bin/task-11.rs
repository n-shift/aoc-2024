const INPUT: &str = include_str!("../../input/11.txt");

type Stones = Vec<u64>;
use std::collections::HashMap;

fn blink(stones: Stones) -> Stones {
    let mut blinked: Stones = stones;
    let mut buffer: Stones = Vec::new();
    blinked.iter_mut().for_each(|stone| {
        if *stone == 0 {
            *stone = 1;
        } else {
            let engraved = stone.to_string();
            if engraved.len() % 2 == 0 {
                let (a, b) = engraved.split_at(engraved.len() / 2);
                *stone = a.parse::<u64>().unwrap();
                buffer.push(b.parse::<u64>().unwrap());
            } else {
                *stone *= 2024;
            }
        }
    });
    blinked.extend(buffer);
    blinked
}

fn sum_blink_cached(cur: u64, to_iter: usize, cache: &mut HashMap<(u64, usize), u64>) -> usize {
    if to_iter == 0 {
        return 1;
    }
    if let Some(entry) = cache.get(&(cur, to_iter)) {
        return *entry as usize;
    }
    let amount = blink(vec![cur])
        .iter()
        .map(|v| sum_blink_cached(*v, to_iter - 1, cache))
        .sum::<usize>();
    cache.insert((cur, to_iter), amount as u64);
    amount
}

fn main() {
    let stones: Stones = INPUT
        .replace('\n', "")
        .split(' ')
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<_>();
    let mut cache: HashMap<(u64, usize), u64> = HashMap::new();
    let cached_25 = stones
        .iter()
        .map(|s| sum_blink_cached(*s, 25, &mut cache))
        .sum::<usize>();
    println!("After 25: {cached_25}");
    let cached_75 = stones
        .iter()
        .map(|s| sum_blink_cached(*s, 75, &mut cache))
        .sum::<usize>();
    println!("After 75: {cached_75}");
}
