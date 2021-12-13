use std::collections::HashSet;

#[derive(Debug)]
enum Axis {
    X,
    Y,
}

type Point = (u32, u32);
type Line = (Axis, u32);

pub fn process(input: &str) -> Option<u32> {
    let (points_str, folds_str) = input.split_once("\n\n")?;
    let mut points: Vec<Point> = points_str
        .lines()
        .map(|l| {
            let (x_str, y_str) = l.split_once(',').unwrap();
            (x_str.parse::<u32>().unwrap(), y_str.parse::<u32>().unwrap())
        })
        .collect();

    let folds: Vec<Line> = folds_str
        .lines()
        .map(|l| {
            let mut f = l.split(' ');
            let s = f.nth(2).unwrap();
            let (axis_str, num_str) = s.split_once('=').unwrap();
            let axis = match axis_str {
                "x" => Axis::X,
                "y" => Axis::Y,
                _ => panic!("invalid axis"),
            };

            (axis, num_str.parse::<u32>().unwrap())
        })
        .collect();

    fold(&mut points, &folds[0]);

    let set: HashSet<Point> = HashSet::from_iter(points.iter().cloned());

    Some(set.len() as u32)
}

pub fn process_pt2(input: &str) {
    let (points_str, folds_str) = input.split_once("\n\n").unwrap();
    let mut points: Vec<Point> = points_str
        .lines()
        .map(|l| {
            let (x_str, y_str) = l.split_once(',').unwrap();
            (x_str.parse::<u32>().unwrap(), y_str.parse::<u32>().unwrap())
        })
        .collect();

    let folds: Vec<Line> = folds_str
        .lines()
        .map(|l| {
            let mut f = l.split(' ');
            let s = f.nth(2).unwrap();
            let (axis_str, num_str) = s.split_once('=').unwrap();
            let axis = match axis_str {
                "x" => Axis::X,
                "y" => Axis::Y,
                _ => panic!("invalid axis"),
            };

            (axis, num_str.parse::<u32>().unwrap())
        })
        .collect();

    for i in folds.iter() {
        fold(&mut points, i);
        let set: HashSet<Point> = HashSet::from_iter(points.iter().cloned());
        points = Vec::from_iter(set.iter().cloned());
    }

    let max_x = points.iter().max_by(|&x, &y| x.0.cmp(&y.0)).unwrap().0;
    let max_y = points.iter().max_by(|&x, &y| x.1.cmp(&y.1)).unwrap().1;

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; (max_x + 1) as usize]; (max_y + 1) as usize];

    for p in points.iter() {
        grid[p.1 as usize][p.0 as usize] = '█';
    }

    for (x, _) in grid.iter().enumerate() {
        for y in grid[x].iter() {
            print!("{}", y);
        }
        println!()
    }
}

fn fold(points: &mut Vec<Point>, line: &Line) {
    let mut points_to_remove: Vec<usize> = Vec::new();

    match line.0 {
        Axis::X => {
            *points = points
                .iter()
                .enumerate()
                .map(|(idx, &(x, y))| {
                    if x == line.1 {
                        points_to_remove.push(idx);
                        (x, y)
                    } else if x > line.1 {
                        ((line.1 * 2) - x, y)
                    } else {
                        (x, y)
                    }
                })
                .collect();
        }
        Axis::Y => {
            *points = points
                .iter()
                .enumerate()
                .map(|(idx, &(x, y))| {
                    if y == line.1 {
                        points_to_remove.push(idx);
                        (x, y)
                    } else if y > line.1 {
                        (x, (line.1 * 2) - y)
                    } else {
                        (x, y)
                    }
                })
                .collect();
        }
    }

    let mut new_points: Vec<Point> = Vec::new();

    for (idx, &p) in points.iter().enumerate() {
        if !points_to_remove.contains(&idx) {
            new_points.push(p);
        }
    }

    *points = new_points;
}
