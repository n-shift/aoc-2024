const INPUT: &str = include_str!("../../input/11.txt");

type Stones = Vec<u64>;

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

fn main() {
    let stones: Stones = INPUT.replace('\n', "").split(' ').map(|n| n.parse::<u64>().unwrap()).collect::<_>();
    println!("Before: {stones:?}");
    let blinked = (0..25).fold(stones.clone(), |acc, _| blink(acc));
    println!("Amount: {}", blinked.len());
}
