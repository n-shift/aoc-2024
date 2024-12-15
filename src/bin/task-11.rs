const INPUT: &str = include_str!("../../input/11.txt");

type Stones = Vec<u64>;
type Cache = std::collections::HashMap<(u64, usize), u64>;

fn blink(stone: u64) -> Stones {
    if stone == 0 {
        vec![1]
    } else {
        let engraved = stone.to_string();
        if engraved.len() % 2 == 0 {
            let (a, b) = engraved.split_at(engraved.len() / 2);
            vec![a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()]
        } else {
            vec![stone * 2024]
        }
    }
}

fn sum_blink_cached(cur: u64, to_iter: usize, cache: &mut Cache) -> usize {
    if to_iter == 0 {
        return 1;
    }
    if let Some(entry) = cache.get(&(cur, to_iter)) {
        return *entry as usize;
    }
    let amount = blink(cur)
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
    let mut cache: Cache = std::collections::HashMap::new();
    let (cached_25, cached_75) = stones
        .iter()
        .map(|s| {
            (
                sum_blink_cached(*s, 25, &mut cache),
                sum_blink_cached(*s, 75, &mut cache),
            )
        })
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
    println!("After 25: {cached_25}");
    println!("After 75: {cached_75}");
}
