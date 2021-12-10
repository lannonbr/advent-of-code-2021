pub fn process(input: &str) -> Option<u32> {
    let points: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let small_numbers = get_small_numbers(&points);

    Some(small_numbers.iter().map(|&(e, _)| e).sum())
}

fn get_small_numbers(points: &Vec<Vec<u32>>) -> Vec<(u32, (usize, usize))> {
    let mut small_numbers: Vec<(u32, (usize, usize))> = Vec::new();

    for (x, _) in points.iter().enumerate() {
        for (y, &entry) in points[x].iter().enumerate() {
            let mut fail = false;

            // left
            if x != 0 {
                if points[x - 1][y] <= points[x][y] {
                    fail = true;
                }
            }

            // right
            if x < points.len() - 1 {
                if points[x + 1][y] <= points[x][y] {
                    fail = true
                }
            }

            // up
            if y != 0 {
                if points[x][y - 1] <= points[x][y] {
                    fail = true
                }
            }

            // down
            if y < points[x].len() - 1 {
                if points[x][y + 1] <= points[x][y] {
                    fail = true
                }
            }

            if fail == false {
                small_numbers.push((entry + 1, (x, y)));
            }
        }
    }

    small_numbers
}

pub fn process_pt2(input: &str) -> Option<u32> {
    let points: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let small_numbers: Vec<(u32, (usize, usize))> = get_small_numbers(&points);

    Some(expand(small_numbers, points))
}

fn expand(numbers: Vec<(u32, (usize, usize))>, map: Vec<Vec<u32>>) -> u32 {
    let mut basin_sizes: Vec<u32> = Vec::new();

    for (_, (x, y)) in numbers {
        let count = check_edges(&mut vec![(500, 500)], vec![(x, y)], &map) + 1;

        basin_sizes.push(count)
    }

    basin_sizes.sort();
    basin_sizes.reverse();

    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}

fn check_edges(
    visited_points: &mut Vec<(usize, usize)>,
    points: Vec<(usize, usize)>,
    map: &Vec<Vec<u32>>,
) -> u32 {
    let mut check_nums: Vec<(usize, usize)> = Vec::new();

    let mut c = 0;

    for point in points {
        let (x, y) = point;

        if x != 0 && !visited(&visited_points, (x - 1, y)) {
            if map[x - 1][y] != 9 {
                c += 1;
                check_nums.push((x - 1, y));
                visited_points.push((x - 1, y));
            }
        }

        if x < map.len() - 1 && !visited(&visited_points, (x + 1, y)) {
            if map[x + 1][y] != 9 {
                c += 1;
                check_nums.push((x + 1, y));
                visited_points.push((x + 1, y));
            }
        }

        if y != 0 && !visited(&visited_points, (x, y - 1)) {
            if map[x][y - 1] != 9 {
                c += 1;
                check_nums.push((x, y - 1));
                visited_points.push((x, y - 1));
            }
        }

        if y < map[x].len() - 1 && !visited(&visited_points, (x, y + 1)) {
            if map[x][y + 1] != 9 {
                c += 1;
                check_nums.push((x, y + 1));
                visited_points.push((x, y + 1));
            }
        }

        visited_points.push((x, y));

        if check_nums.len() != 0 {
            c += check_edges(visited_points, check_nums.clone(), map);
        }
    }

    c
}

fn visited(visited_points: &Vec<(usize, usize)>, point: (usize, usize)) -> bool {
    for vp in visited_points {
        if vp.0 == point.0 && vp.1 == point.1 {
            return true;
        }
    }

    false
}
