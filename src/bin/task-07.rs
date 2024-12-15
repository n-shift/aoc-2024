const INPUT: &str = include_str!("../../input/7.txt");

fn format_radix(mut x: u64, radix: u64) -> String {
    let mut result = vec![];

    loop {
        let m = (x % radix) as u32;
        x /= radix;

        result.push(std::char::from_digit(m, radix as u32).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}

fn calibrate(concat: bool) -> u64 {
    INPUT
        .split('\n')
        .map(|l| {
            if l.is_empty() {
                return 0;
            }
            let (desired, given) = l.split_once(':').unwrap();
            let desired = desired.parse::<u64>().unwrap();
            let given = given
                .split(' ')
                .filter(|c| !c.is_empty())
                .map(|c| c.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            let positions = given.len() - 1;
            let perms: u64 = (2u64 + concat as u64).pow(positions as u32);
            if (0..perms).any(|p| {
                let mut operators = format!("{:0>positions$}", format_radix(p, 2 + concat as u64))
                    .chars()
                    .collect::<Vec<char>>();
                desired
                    == given
                        .clone()
                        .into_iter()
                        .reduce(|acc, n| match operators.pop().unwrap() {
                            '0' => acc + n,
                            '1' => acc * n,
                            '2' => acc * 10u64.pow(n.ilog10() + 1) + n,
                            _ => panic!("how."),
                        })
                        .unwrap()
            }) {
                desired
            } else {
                0
            }
        })
        .sum::<u64>()
}
fn main() {
    println!("W/o concat: {}", calibrate(false));
    println!("W/ concat: {}", calibrate(true));
}
