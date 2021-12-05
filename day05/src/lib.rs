use nom::{bytes::complete::tag, character::complete::u64};

#[derive(Debug)]
struct Line {
    x1: u64,
    y1: u64,
    x2: u64,
    y2: u64,
}

fn parse_line(input: &str) -> nom::IResult<&str, Line> {
    let (input, x1) = u64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y1) = u64(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, x2) = u64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y2) = u64(input)?;

    Ok((input, Line { x1, y1, x2, y2 }))
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.x1 != self.x2 && self.y1 == self.y2
    }
    fn is_vertical(&self) -> bool {
        self.x1 == self.x2 && self.y1 != self.y2
    }
}

pub fn process(input: &str) -> Option<u64> {
    let mut lines: Vec<Line> = input
        .lines()
        .map(|l| {
            let (_, line) = parse_line(l).unwrap();
            line
        })
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .collect();

    let mut grid: Vec<Vec<u64>> = vec![vec![0; 1000]; 1000];

    for l in lines.iter_mut() {
        if l.is_horizontal() {
            if l.x1 > l.x2 {
                for c in l.x2..(l.x1 + 1) {
                    grid[c as usize][l.y1 as usize] += 1;
                }
            } else {
                for c in l.x1..(l.x2 + 1) {
                    grid[c as usize][l.y1 as usize] += 1;
                }
            }
        } else if l.is_vertical() {
            if l.y1 > l.y2 {
                for c in l.y2..(l.y1 + 1) {
                    grid[l.x1 as usize][c as usize] += 1;
                }
            } else {
                for c in l.y1..(l.y2 + 1) {
                    grid[l.x1 as usize][c as usize] += 1;
                }
            }
        } else {
            panic!("What are you doing here")
        }
    }

    let mut c = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[x][y] >= 2 {
                c += 1;
            }
        }
    }

    Some(c)
}

pub fn process_pt2(input: &str) -> Option<u64> {
    let mut lines: Vec<Line> = input
        .lines()
        .map(|l| {
            let (_, line) = parse_line(l).unwrap();
            line
        })
        .collect();

    let mut grid: Vec<Vec<u64>> = vec![vec![0; 1000]; 1000];

    for l in lines.iter_mut() {
        if l.is_horizontal() {
            if l.x1 > l.x2 {
                for c in l.x2..(l.x1 + 1) {
                    grid[c as usize][l.y1 as usize] += 1;
                }
            } else {
                for c in l.x1..(l.x2 + 1) {
                    grid[c as usize][l.y1 as usize] += 1;
                }
            }
        } else if l.is_vertical() {
            if l.y1 > l.y2 {
                for c in l.y2..(l.y1 + 1) {
                    grid[l.x1 as usize][c as usize] += 1;
                }
            } else {
                for c in l.y1..(l.y2 + 1) {
                    grid[l.x1 as usize][c as usize] += 1;
                }
            }
        } else {
            if l.x1 > l.x2 && l.y1 < l.y2 {
                // left, down

                let mut start = (l.x1, l.y1);
                let mut i = 0;
                let dist = l.x1 - l.x2;
                while i < (dist + 1) {
                    grid[start.0 as usize][start.1 as usize] += 1;
                    if start.0 != 0 {
                        start.0 -= 1;
                    }
                    start.1 += 1;
                    i += 1;
                }
            } else if l.x1 > l.x2 && l.y1 > l.y2 {
                // left, up
                let mut start = (l.x1, l.y1);
                let mut i = 0;
                let dist = l.x1 - l.x2;
                while i < (dist + 1) {
                    grid[start.0 as usize][start.1 as usize] += 1;
                    start.0 -= 1;
                    if start.1 != 0 {
                        start.1 -= 1;
                    }
                    i += 1;
                }
            } else if l.x1 < l.x2 && l.y1 < l.y2 {
                // right, down
                let mut start = (l.x1, l.y1);
                let mut i = 0;
                let dist = l.x2 - l.x1;
                while i < (dist + 1) {
                    grid[start.0 as usize][start.1 as usize] += 1;
                    start.0 += 1;
                    start.1 += 1;
                    i += 1;
                }
            } else if l.x1 < l.x2 && l.y1 > l.y2 {
                // right up
                let mut start = (l.x1, l.y1);
                let mut i = 0;
                let dist = l.x2 - l.x1;
                while i < (dist + 1) {
                    grid[start.0 as usize][start.1 as usize] += 1;
                    start.0 += 1;
                    if start.1 != 0 {
                        start.1 -= 1;
                    }
                    i += 1;
                }
            }
        }
    }

    let mut c = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[x][y] > 1 {
                c += 1;
            }
        }
    }

    Some(c)
}
