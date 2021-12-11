use std::collections::HashMap;

pub fn process(input: &str, steps: usize) -> Option<u32> {
    let mut lines: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut total_flashes = 0;

    for _ in 0..steps {
        let number_of_flashes = step(&mut lines);

        total_flashes += number_of_flashes;
    }

    Some(total_flashes as u32)
}

pub fn process_pt2(input: &str, steps: usize) -> Option<u32> {
    let mut lines: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    for i in 0..steps {
        step(&mut lines);

        let mut all_zeros = true;
        'outer: for x1 in 0..lines.len() {
            for y1 in 0..lines[x1].len() {
                if lines[x1][y1] != 0 {
                    all_zeros = false;
                    break 'outer;
                }
            }
        }
        if all_zeros {
            return Some((i + 1) as u32);
        }
    }

    None
}

fn step(input: &mut Vec<Vec<u32>>) -> usize {
    let mut flashed_map: HashMap<(i32, i32), bool> = HashMap::new();
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            if input[x][y] != 9 {
                if flashed_map.get(&(x as i32, y as i32)).is_none() {
                    input[x][y] += 1;
                }
            } else {
                if flashed_map.get(&(x as i32, y as i32)).is_none() {
                    flashed_map.insert((x as i32, y as i32), true);
                    flash(input, x as i32, y as i32, &mut flashed_map);
                }
            }
        }
    }

    flashed_map.values().len()
}

fn flash(input: &mut Vec<Vec<u32>>, x: i32, y: i32, flashed_map: &mut HashMap<(i32, i32), bool>) {
    input[x as usize][y as usize] = 0;
    let mut x0: i32 = -1;
    let mut y0: i32 = -1;

    while x0 < 2 {
        while y0 < 2 {
            if (x + x0 != -1 && ((x + x0) as usize) < input.len())
                && (y + y0 != -1 && ((y + y0) as usize) < input[(x + x0) as usize].len())
            {
                if input[(x + x0) as usize][(y + y0) as usize] == 0
                    && flashed_map.get(&(x + x0, y + y0)).is_some()
                {
                } else if input[(x + x0) as usize][(y + y0) as usize] != 9 {
                    input[(x + x0) as usize][(y + y0) as usize] += 1;
                } else {
                    if flashed_map.get(&(x + x0, y + y0)).is_none() {
                        flashed_map.insert((x + x0, y + y0), true);
                        flash(input, x + x0, y + y0, flashed_map);
                    }
                }
            }
            y0 += 1;
        }
        x0 += 1;
        y0 = -1;
    }
}
