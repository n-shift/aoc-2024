const BOUNDS: (isize, isize) = (101, 103);
const INPUT: &str = include_str!("../../input/14.txt");

struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl Robot {
    fn init(line: &str) -> Self {
        let [position, velocity] = line
            .split(' ')
            .map(|x| {
                let t = x[2..]
                    .split(',')
                    .map(|n| n.parse::<isize>().unwrap())
                    .collect::<Vec<_>>();
                (t[0], t[1])
            })
            .collect::<Vec<_>>()[..]
        else {
            panic!("how.")
        };
        Self { position, velocity }
    }
    fn simulate(&mut self) {
        let mut npx = self.position.0 + self.velocity.0;
        if npx < 0 {
            npx += BOUNDS.0;
        }
        if npx >= BOUNDS.0 {
            npx -= BOUNDS.0;
        }
        let mut npy = self.position.1 + self.velocity.1;
        if npy < 0 {
            npy += BOUNDS.1;
        }
        if npy >= BOUNDS.1 {
            npy -= BOUNDS.1;
        }
        self.position = (npx, npy);
    }
    fn quadrant(&self, quadrants: &mut [usize; 4]) {
        if self.position.0 < BOUNDS.0 / 2 && self.position.1 < BOUNDS.1 / 2 {
            quadrants[0] += 1;
        }
        if self.position.0 < BOUNDS.0 / 2
            && BOUNDS.1 / 2 < self.position.1
            && self.position.1 < BOUNDS.1
        {
            quadrants[1] += 1;
        }
        if BOUNDS.0 / 2 < self.position.0
            && self.position.0 < BOUNDS.0
            && self.position.1 < BOUNDS.1 / 2
        {
            quadrants[2] += 1;
        }
        if BOUNDS.0 / 2 < self.position.0
            && self.position.0 < BOUNDS.0
            && BOUNDS.1 / 2 < self.position.1
            && self.position.1 < BOUNDS.1
        {
            quadrants[3] += 1;
        }
    }
}

fn flush(robots: &[Robot], timestamp: usize) {
    let mut output = [['0'; BOUNDS.0 as usize]; BOUNDS.1 as usize];
    robots
        .iter()
        .for_each(|r| output[r.position.1 as usize][r.position.0 as usize] = '1');
    let output = output
        .iter()
        .map(|l| l.iter().collect::<String>())
        .collect::<Vec<_>>();
    if output.iter().any(|l| l.contains("1111111111")) {
        println!("t={timestamp}s");
        output.iter().for_each(|l| println!("{l}"));
    }
}

fn main() {
    let mut robots: Vec<Robot> = INPUT.lines().map(Robot::init).collect();
    robots
        .iter_mut()
        .for_each(|r| (0..100).for_each(|_| r.simulate()));
    let mut quadrants = [0; 4];
    robots.iter().for_each(|r| r.quadrant(&mut quadrants));
    let safety: usize = quadrants.iter().product();
    println!("Safety factor: {safety}");
    // prints out an easter egg :P
    let mut robots: Vec<Robot> = INPUT.lines().map(Robot::init).collect();
    (1..=10000).for_each(|t| {
        robots.iter_mut().for_each(|r| r.simulate());
        flush(&robots, t)
    });
}
