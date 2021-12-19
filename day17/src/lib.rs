use nom::{bytes::complete::tag, character::complete::i64, sequence::separated_pair, IResult};

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

fn parse_target(input: &str) -> IResult<&str, Target> {
    let (input, _) = tag("target area: x=")(input)?;
    let (input, (x0, x1)) = separated_pair(i64, tag(".."), i64)(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, (y0, y1)) = separated_pair(i64, tag(".."), i64)(input)?;

    let target = Target {
        x0: x0 as isize,
        x1: x1 as isize,
        y0: y0 as isize,
        y1: y1 as isize,
    };
    Ok((input, target))
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
    let (_, target) = parse_target(input).unwrap();

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
    let (_, target) = parse_target(input).unwrap();

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
