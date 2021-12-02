pub fn process(input: &str) -> i32 {
    let lines = input.lines();

    let sections: Vec<(&str, i32)> = lines
        .map(|line| {
            let sub: Vec<&str> = line.split(' ').collect();
            (sub[0], sub[1].parse::<i32>().unwrap())
        })
        .collect();

    let mut y_pos = 0;
    let mut z_pos = 0;

    for (dir, len) in sections {
        if dir == "forward" {
            z_pos += len;
        } else if dir == "up" {
            y_pos -= len;
        } else if dir == "down" {
            y_pos += len;
        }
    }

    y_pos * z_pos
}

pub fn process_pt2(input: &str) -> i32 {
    let lines = input.lines();

    let sections: Vec<(&str, i32)> = lines
        .map(|line| {
            let sub: Vec<&str> = line.split(' ').collect();
            (sub[0], sub[1].parse::<i32>().unwrap())
        })
        .collect();

    let mut y_pos = 0;
    let mut z_pos = 0;
    let mut aim = 0;

    for (dir, len) in sections {
        if dir == "forward" {
            z_pos += len;
            y_pos += aim * len;
        } else if dir == "up" {
            aim -= len;
        } else if dir == "down" {
            aim += len;
        }
    }

    y_pos * z_pos
}
