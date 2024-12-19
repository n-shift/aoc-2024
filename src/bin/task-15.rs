const INPUT: &str = include_str!("../../input/15.txt");

use std::{cmp, convert::From};
#[derive(PartialEq, Clone, Copy)]
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
fn delta(before: (usize, usize), dir: Dir) -> (usize, usize) {
    let (x, y) = before;
    match dir {
        Dir::Right => (x + 1, y),
        Dir::Left => (x - 1, y),
        Dir::Down => (x, y + 1),
        Dir::Up => (x, y - 1),
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
        grid[y][x] = Cell::Nothing;
        let (x, y) = delta((x, y), dir);
        grid[y][x] = Cell::Robot;
        if queue.0 != queue.1 {
            let (lx, ly) = delta(queue.1, dir);
            grid[ly][lx] = Cell::Box;
        }
    }
    fn queue(&self, dir: Dir) -> Option<((usize, usize), (usize, usize))> {
        let mut queue = ((0, 0), (0, 0));
        let &Some((y, Some(x))) = &self
            .0
            .iter()
            .map(|r| r.iter().position(|c| *c == Cell::Robot))
            .enumerate()
            .find(|(_, r)| r.is_some())
        else {
            unreachable!()
        };
        queue.0 = (x, y);
        let c = |(f, c): &(usize, &Cell)| if **c == Cell::Box { Some(*f) } else { None };
        let t = match dir {
            Dir::Down | Dir::Up => &self.0.iter().map(|r| r[x]).collect(),
            Dir::Right | Dir::Left => &self.0[y],
        };
        let mut q = t.iter().enumerate();
        let q: Vec<(usize, &Cell)> = match dir {
            Dir::Down => {
                q.nth(y);
                q.collect()
            }
            Dir::Up => q.take(y).rev().collect(),
            Dir::Right => {
                q.nth(x);
                q.collect()
            }
            Dir::Left => q.take(x).rev().collect(),
        };
        let wrap = q.iter().map(c).take_while(Option::is_some).flatten().last();
        queue.1 = match dir {
            Dir::Right => (cmp::max(x, wrap.unwrap_or(x)), y),
            Dir::Left => (cmp::min(x, wrap.unwrap_or(x)), y),
            Dir::Down => (x, cmp::max(y, wrap.unwrap_or(y))),
            Dir::Up => (x, cmp::min(y, wrap.unwrap_or(y))),
        };

        let (fx, fy) = delta(queue.1, dir);
        if self.0[fy][fx] == Cell::Wall {
            None
        } else {
            Some(queue)
        }
    }
    fn gps(&self) -> usize {
        self.0
            .iter()
            .map(|r| {
                r.iter()
                    .enumerate()
                    .filter(|(_, c)| **c == Cell::Box)
                    .fold((0, 0), |acc, (x, _)| (acc.0 + x, acc.1 + 1))
            })
            .enumerate()
            .fold(0, |acc, (y, (x, n))| acc + 100 * y * n + x)
    }
}

fn main() {
    let [raw_grid, raw_inst] = INPUT.split("\n\n").collect::<Vec<_>>()[..] else { unreachable!() };
    let mut grid = Grid(
        raw_grid
            .lines()
            .map(|l| l.chars().map(Cell::from).collect::<Vec<Cell>>())
            .collect::<Vec<_>>(),
    );
    raw_inst.chars().filter(|c| *c != '\n').for_each(|c| grid.shift(c.into()));
    println!("GPS sum: {}", grid.gps());
}
