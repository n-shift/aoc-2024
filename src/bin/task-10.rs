const INPUT: &str = include_str!("../../input/10.txt");
fn get_trailheads(
    (row, col): (usize, usize),
    height: u8,
    map: &Vec<Vec<u8>>,
    visited: &mut Vec<(usize, usize)>,
) -> usize {
    if height == 0 && !visited.contains(&(row, col)) {
        visited.push((row, col));
        return 1;
    }

    let mut result = 0;

    if row > 0 && map[row - 1][col] == height - 1 {
        result += get_trailheads((row - 1, col), height - 1, map, visited);
    }

    if row + 1 < map.len() && map[row + 1][col] == height - 1 {
        result += get_trailheads((row + 1, col), height - 1, map, visited);
    }

    if col > 0 && map[row][col - 1] == height - 1 {
        result += get_trailheads((row, col - 1), height - 1, map, visited);
    }

    if col + 1 < map[0].len() && map[row][col + 1] == height - 1 {
        result += get_trailheads((row, col + 1), height - 1, map, visited);
    }

    result
}

fn count_paths((row, col): (usize, usize), height: u8, map: &Vec<Vec<u8>>) -> usize {
    if height == 0 {
        return 1;
    }

    let mut result = 0;

    if row > 0 && map[row - 1][col] == height - 1 {
        result += count_paths((row - 1, col), height - 1, map);
    }

    if row + 1 < map.len() && map[row + 1][col] == height - 1 {
        result += count_paths((row + 1, col), height - 1, map);
    }

    if col > 0 && map[row][col - 1] == height - 1 {
        result += count_paths((row, col - 1), height - 1, map);
    }

    if col + 1 < map[0].len() && map[row][col + 1] == height - 1 {
        result += count_paths((row, col + 1), height - 1, map);
    }

    result
}

fn main() {
    let iter = INPUT.lines().map(|l| l.bytes().map(|b| b - b'0').collect::<Vec<u8>>());
    let map = iter.clone().collect::<Vec<Vec<u8>>>();
    let (score, rate) = iter.enumerate().fold((0, 0), |acc, (r_id, r)| {
        let new = r.iter().enumerate().filter(|(_, c)| **c == 9).fold((0, 0), |acc, (c_id, _)| {
            let mut visited: Vec<(usize, usize)> = Vec::new();
            (
                acc.0 + get_trailheads((r_id, c_id), 9, &map, &mut visited),
                acc.1 + count_paths((r_id, c_id), 9, &map),
            )
        });
        (acc.0 + new.0, acc.1 + new.1)
    });
    println!("Score: {score}");
    println!("Rate: {rate}");
}
