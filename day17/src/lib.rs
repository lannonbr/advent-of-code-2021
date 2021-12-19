#[derive(Debug)]
struct Target {
    x0: isize,
    x1: isize,
    y0: isize,
    y1: isize,
}

enum RunResult {
    Contained(isize),
    Invalid,
}

type Velocity = (isize, isize);

impl Target {
    fn within(&self, x: isize, y: isize) -> bool {
        x >= self.x0 && x <= self.x1 && y >= self.y0 && y <= self.y1
    }

    fn past(&self, x: isize, y: isize) -> bool {
        x > self.x1 || y < self.y0
    }
}

fn run_vel(t: &Target, vel: Velocity) -> RunResult {
    let mut x: isize = 0;
    let mut y: isize = 0;

    let mut vel_x = vel.0;
    let mut vel_y = vel.1;

    let mut max_height: isize = 0;

    loop {
        x += vel_x;
        y += vel_y;

        if vel_x < 0 {
            vel_x += 1;
        } else if vel_x > 0 {
            vel_x -= 1;
        } else {
            // X is at 0, don't do anything
        }

        vel_y -= 1;

        if y > max_height {
            max_height = y;
        }

        if t.within(x, y) {
            return RunResult::Contained(max_height);
        }

        if t.past(x, y) {
            return RunResult::Invalid;
        }
    }
}

pub fn process(input: &str) -> Option<i64> {
    let s: Vec<&str> = input.split_whitespace().collect();

    let x_range = s[2]
        .split_once('=')
        .unwrap()
        .1
        .split_once(",")
        .unwrap()
        .0
        .split_once("..")
        .unwrap();
    let y_range = s[3].split_once('=').unwrap().1.split_once("..").unwrap();

    let x0 = isize::from_str_radix(x_range.0, 10).unwrap();
    let x1 = isize::from_str_radix(x_range.1, 10).unwrap();
    let y0 = isize::from_str_radix(y_range.0, 10).unwrap();
    let y1 = isize::from_str_radix(y_range.1, 10).unwrap();

    let target = Target { x0, x1, y0, y1 };

    let mut results: Vec<(Velocity, isize)> = vec![];

    for test_x in 1..500 {
        for test_y in 0..500 {
            match run_vel(&target, (test_x, test_y)) {
                RunResult::Contained(max_height) => {
                    results.push(((test_x, test_y), max_height));
                }
                RunResult::Invalid => {}
            }
        }
    }

    let top = results.iter().max_by(|&a, &b| a.1.cmp(&b.1)).unwrap();

    Some(top.1 as i64)
}

pub fn process_pt2(input: &str) -> Option<u64> {
    let s: Vec<&str> = input.split_whitespace().collect();

    let x_range = s[2]
        .split_once('=')
        .unwrap()
        .1
        .split_once(",")
        .unwrap()
        .0
        .split_once("..")
        .unwrap();
    let y_range = s[3].split_once('=').unwrap().1.split_once("..").unwrap();

    let x0 = isize::from_str_radix(x_range.0, 10).unwrap();
    let x1 = isize::from_str_radix(x_range.1, 10).unwrap();
    let y0 = isize::from_str_radix(y_range.0, 10).unwrap();
    let y1 = isize::from_str_radix(y_range.1, 10).unwrap();

    let target = Target { x0, x1, y0, y1 };

    let mut results: Vec<(Velocity, isize)> = vec![];

    for test_x in 1..5000 {
        for test_y in -5000..5000 {
            match run_vel(&target, (test_x, test_y)) {
                RunResult::Contained(max_height) => {
                    results.push(((test_x, test_y), max_height));
                }
                RunResult::Invalid => {}
            }
        }
    }

    Some(results.len() as u64)
}
