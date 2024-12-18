const INPUT: &str = include_str!("../../input/15.txt");

use std::convert::From;
#[derive(PartialEq)]
enum Cell {
    Robot,
    Box,
    Wall,
    Nothing,
}

#[derive(Copy, Clone)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Cell {
    fn from(item: char) -> Self {
        match item {
            '@' => Self::Robot,
            'O' => Self::Box,
            '.' => Self::Nothing,
            '#' => Self::Wall,
            _ => unreachable!(),
        }
    }
}

impl From<char> for Dir {
    fn from(item: char) -> Self {
        match item {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => unreachable!(),
        }
    }
}

struct Grid(Vec<Vec<Cell>>);
impl From<Vec<Vec<Cell>>> for Grid {
    fn from(item: Vec<Vec<Cell>>) -> Self {
        Self(item)
    }
}

impl Grid {
    fn shift(&mut self, dir: Dir) {
        let queue = self.queue(dir);
        if queue.is_none() {
            return;
        }
        let queue = queue.unwrap();
        let grid = &mut self.0;
        let (x, y) = queue.0;
        let (lx, ly) = queue.1;
        grid[y][x] = Cell::Nothing;
        match dir {
            Dir::Up => {
                grid[y - 1][x] = Cell::Robot;
                if queue.0 != queue.1 {
                    grid[ly - 1][x] = Cell::Box;
                }
            }
            Dir::Right => {
                grid[y][x + 1] = Cell::Robot;
                if queue.0 != queue.1 {
                    grid[y][lx + 1] = Cell::Box;
                }
            }
            Dir::Down => {
                grid[y + 1][x] = Cell::Robot;
                if queue.0 != queue.1 {
                    grid[ly + 1][x] = Cell::Box;
                }
            }
            Dir::Left => {
                grid[y][x - 1] = Cell::Robot;
                if queue.0 != queue.1 {
                    grid[y][lx - 1] = Cell::Box;
                }
            }
        }
    }
    fn queue(&self, dir: Dir) -> Option<((usize, usize), (usize, usize))> {
        let mut queue = ((0, 0), (0, 0));
        let grid = &self.0;
        let Some((y, Some(x))) = grid
            .iter()
            .map(|r| r.iter().position(|c| *c == Cell::Robot))
            .enumerate()
            .find(|(_, r)| r.is_some())
        else {
            unreachable!()
        };
        queue.0 = (x, y);
        match dir {
            Dir::Down => {
                let mut q = grid.iter().enumerate();
                q.nth(y);
                let wrap = q
                    .map(|(fy, r)| if r[x] == Cell::Box { Some(fy) } else { None })
                    .take_while(Option::is_some)
                    .flatten()
                    .last();
                queue.1 = if let Some(fy) = wrap {
                    (x, fy)
                } else {
                    queue.0
                };
            }
            Dir::Up => {
                let wrap = grid
                    .iter()
                    .enumerate()
                    .take(y)
                    .rev()
                    .map(|(fy, r)| if r[x] == Cell::Box { Some(fy) } else { None })
                    .take_while(Option::is_some)
                    .flatten()
                    .last();
                queue.1 = if let Some(fy) = wrap {
                    (x, fy)
                } else {
                    queue.0
                };
            }
            Dir::Right => {
                let mut q = grid[y].iter().enumerate();
                q.nth(x);
                let wrap = q
                    .map(|(fx, c)| if *c == Cell::Box { Some(fx) } else { None })
                    .take_while(Option::is_some)
                    .flatten()
                    .last();
                queue.1 = if let Some(fx) = wrap {
                    (fx, y)
                } else {
                    queue.0
                };
            }
            Dir::Left => {
                let wrap = grid[y]
                    .iter()
                    .enumerate()
                    .take(x)
                    .rev()
                    .map(|(fx, c)| if *c == Cell::Box { Some(fx) } else { None })
                    .take_while(Option::is_some)
                    .flatten()
                    .last();
                queue.1 = if let Some(fx) = wrap {
                    if fx < x {
                        (fx, y)
                    } else {
                        queue.0
                    }
                } else {
                    queue.0
                };
            }
        }
        let (fx, fy) = queue.1;
        match dir {
            Dir::Right => {
                if self.0[fy][fx + 1] == Cell::Wall {
                    None
                } else {
                    Some(queue)
                }
            }
            Dir::Left => {
                if self.0[fy][fx - 1] == Cell::Wall {
                    None
                } else {
                    Some(queue)
                }
            }
            Dir::Up => {
                if self.0[fy - 1][fx] == Cell::Wall {
                    None
                } else {
                    Some(queue)
                }
            }
            Dir::Down => {
                if self.0[fy + 1][fx] == Cell::Wall {
                    None
                } else {
                    Some(queue)
                }
            }
        }
    }
    fn gps(&self) -> usize {
        self.0.iter()
            .map(|r| {
                let t = r
                    .iter()
                    .enumerate()
                    .filter(|(_x, c)| **c == Cell::Box)
                    .map(|(x, _c)| x)
                    .collect::<Vec<_>>();
                (t.iter().sum::<usize>(), t.len())
            })
            .enumerate()
            .map(|(y, (x, n))| 100 * y * n + x)
            .sum::<usize>()
    }
}

fn main() {
    let [raw_grid, raw_inst] = INPUT.split("\n\n").collect::<Vec<_>>()[..] else {
        unreachable!()
    };
    let mut grid: Grid = raw_grid
        .split('\n')
        .map(|l| l.chars().map(Cell::from).collect::<Vec<Cell>>())
        .collect::<Vec<_>>()
        .into();
    raw_inst
        .chars()
        .filter(|c| *c != '\n')
        .map(Dir::from)
        .for_each(|d| grid.shift(d));
    println!("GPS sum: {}", grid.gps());
}
