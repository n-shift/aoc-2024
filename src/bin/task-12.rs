const INPUT: &str = include_str!("../../input/12.txt");
use std::collections::HashSet;

fn is_valid(p: (i32, i32), max_x: i32, max_y: i32) -> bool {
    let (x, y) = p;
    0 <= x && x < max_x && 0 <= y && y < max_y
}

fn flood_fill(grid: &[Vec<char>], p: (i32, i32)) -> HashSet<(i32, i32)> {
    let max_x = grid[0].len() as i32;
    let max_y = grid.len() as i32;

    let crop = grid[p.1 as usize][p.0 as usize];

    let mut visited = HashSet::new();
    let mut to_visit = vec![p];

    while let Some((x, y)) = to_visit.pop() {
        visited.insert((x, y));

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nx = x + dx;
            let ny = y + dy;
            if is_valid((nx, ny), max_x, max_y)
                && grid[ny as usize][nx as usize] == crop
                && !visited.contains(&(nx, ny))
            {
                to_visit.push((nx, ny));
            }
        }
    }

    visited
}

fn perimeter(points: &HashSet<(i32, i32)>) -> u32 {
    let deltas = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    points
        .iter()
        .flat_map(|(x, y)| {
            deltas
                .iter()
                .map(move |(dx, dy)| (x + dx, y + dy))
                .filter(|new_p| !points.contains(new_p))
        })
        .count() as u32
}

fn part_1() {
    let grid: Vec<Vec<char>> = INPUT.lines().map(|l| l.chars().collect()).collect();

    let mut crops: Vec<HashSet<(i32, i32)>> = Vec::new();
    let mut visited = HashSet::new();
    grid.iter().enumerate().for_each(|(y, row)| {
        (0..row.len()).for_each(|x| {
            let p = (x as i32, y as i32);
            if !visited.contains(&p) && !crops.iter().any(|v| v.contains(&p)) {
                let v = flood_fill(&grid, p);
                crops.push(v.clone());
                visited.extend(v);
            }
        })
    });

    println!("Cost: {}", crops.iter().map(|v| (v.len() as u32) * perimeter(v)).sum::<u32>());
}

fn get_boundaries(points: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let deltas = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    points
        .iter()
        .flat_map(|(x, y)| {
            deltas
                .iter()
                .map(move |(dx, dy)| (x + dx, y + dy))
                .filter(|new_p| !points.contains(new_p))
        })
        .collect()
}

fn count_corners(region: &HashSet<(i32, i32)>) -> usize {
    let mut corners = HashSet::new();
    let kernels = [
        [(-1, 0), (0, -1), (-1, -1)],
        [(1, 0), (0, -1), (1, -1)],
        [(-1, 0), (0, 1), (-1, 1)],
        [(1, 0), (0, 1), (1, 1)],
    ];
    region.iter().for_each(|&(px, py)| {
        kernels.iter().enumerate().for_each(|(i, k)| {
            if k.iter().map(|(kx, ky)| (px + kx, py + ky)).all(|v| !region.contains(&v)) {
                corners.insert((px, py, i));
            }
        })
    });

    let inner_kernels =
        [[(-1, 0), (0, -1)], [(-1, 0), (0, 1)], [(1, 0), (0, -1)], [(1, 0), (0, 1)]];
    let mut inner_corners = HashSet::new();
    get_boundaries(region).iter().for_each(|(px, py)| {
        inner_kernels.iter().enumerate().for_each(|(i, k)| {
            let vals: Vec<(i32, i32)> = k.iter().map(|(kx, ky)| (px + kx, py + ky)).collect();
            if vals.iter().all(|v| region.contains(v)) {
                let (dx, dy) = (k[0].0 + k[1].0, k[0].1 + k[1].1);
                if region.contains(&(px + dx, py + dy)) {
                    inner_corners.insert((px + dx, py + dy, i));
                } else {
                    let ((v1x, v1y), (v2x, v2y)) = (vals[0], vals[1]);
                    let (dx, dy) = (v1x - v2x, v1y - v2y);
                    let d1 = [(-dx, 0), (0, dy)];
                    let d2 = [(dx, 0), (0, -dy)];
                    inner_corners.insert((
                        v1x,
                        v1y,
                        inner_kernels.iter().position(|&x| x == d1).unwrap(),
                    ));
                    inner_corners.insert((
                        v2x,
                        v2y,
                        inner_kernels.iter().position(|&x| x == d2).unwrap(),
                    ));
                }
            }
        })
    });

    corners.len() + inner_corners.len()
}

fn part_2() {
    let grid: Vec<Vec<char>> = INPUT.lines().map(|l| l.chars().collect()).collect();

    let mut crops: Vec<HashSet<(i32, i32)>> = Vec::new();
    let mut visited = HashSet::new();
    grid.iter().enumerate().for_each(|(y, row)| {
        (0..row.len()).for_each(|x| {
            let p = (x as i32, y as i32);
            if !visited.contains(&p) {
                let v = flood_fill(&grid, p);
                crops.push(v.clone());
                visited.extend(v);
            }
        })
    });

    println!("Discount: {}", crops.iter().map(|v| v.len() * count_corners(v)).sum::<usize>());
}

fn main() {
    part_1();
    part_2();
}
