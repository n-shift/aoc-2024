const FILE: &str = include_str!("../../input/6.txt");
const SIZE: usize = 130;
const LEN: usize = SIZE * SIZE;

#[derive(PartialEq, Clone, Debug)]
enum Cell {
    Unvisited,
    Visited,
    Wall,
}

#[derive(PartialEq, Copy, Clone)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

fn run(grid: &mut [Cell], mut pos: usize) -> bool {
    let mut dir = Dir::Up;
    let mut prev = Vec::new();
    loop {
        grid[pos] = Cell::Visited;
        if prev.contains(&(pos, dir)) {
            return true;
        }
        let next = match dir {
            Dir::Up if pos / SIZE != 0 => pos - SIZE,
            Dir::Right if pos % SIZE != SIZE - 1 => pos + 1,
            Dir::Down if pos / SIZE != SIZE - 1 => pos + SIZE,
            Dir::Left if pos % SIZE != 0 => pos - 1,
            _ => return false,
        };
        if grid[next] == Cell::Wall {
            prev.push((pos, dir));
            dir = match dir {
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
            };
        } else {
            pos = next;
        }
    }
}

fn main() {
    let input = FILE.replace('\n', "").to_owned();
    let start = input.find('^').unwrap();
    let mut grid = vec![Cell::Unvisited; LEN];
    grid.iter_mut().enumerate().for_each(|(i, c)| {
        if input.as_bytes()[i] == b'#' {
            *c = Cell::Wall
        }
    });
    let mut visited_grid = grid.clone();
    run(&mut visited_grid, start);
    let visited = visited_grid
        .iter()
        .enumerate()
        .filter(|(i, c)| **c == Cell::Visited && *i != start)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    println!(
        "Path length: {}",
        visited_grid
            .iter()
            .fold(0, |acc, c| acc + (*c == Cell::Visited) as usize)
    );
    println!(
        "Obstruction positions: {}",
        visited
            .iter()
            .filter(|i| {
                let mut checking = grid.clone();
                checking[**i] = Cell::Wall;
                run(&mut checking, start)
            })
            .count()
    )
}
