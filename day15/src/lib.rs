use pathfinding::prelude::{dijkstra, Matrix};

pub fn process(input: &str) -> Option<u32> {
    let rows = input.lines().collect::<Vec<&str>>().len();
    let v: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let flat_v: Vec<u32> = v.iter().flatten().map(|c| *c).collect();

    let m = Matrix::from_vec(rows, rows, flat_v).unwrap();

    solve_shortest_path(m)
}

pub fn process_pt2(input: &str) -> Option<u32> {
    let rows = input.lines().collect::<Vec<&str>>().len();
    let v: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut expanded_v: Vec<Vec<u32>> = vec![vec![0; rows * 5]; rows * 5];

    let add_matrix: Vec<Vec<u32>> = vec![
        vec![0, 1, 2, 3, 4],
        vec![1, 2, 3, 4, 5],
        vec![2, 3, 4, 5, 6],
        vec![3, 4, 5, 6, 7],
        vec![4, 5, 6, 7, 8],
    ];

    for (y_idx, y) in v.iter().enumerate() {
        for (x_idx, x) in y.iter().enumerate() {
            expanded_v[y_idx][x_idx] = *x;
            for y0 in 0..5 {
                for x0 in 0..5 {
                    expanded_v[(y_idx + y0 * rows) as usize][(x_idx + x0 * rows) as usize] =
                        if x + add_matrix[y0][x0] > 9 {
                            1 + ((x + add_matrix[y0][x0]) % 10)
                        } else {
                            x + add_matrix[y0][x0]
                        };
                }
            }
        }
    }

    let flat_v: Vec<u32> = expanded_v.iter().flatten().map(|c| *c).collect();

    let m = Matrix::from_vec(rows * 5, rows * 5, flat_v).unwrap();

    solve_shortest_path(m)
}

fn solve_shortest_path(m: Matrix<u32>) -> Option<u32> {
    let result = dijkstra(
        &(0, 0),
        |p| m.neighbours(*p, false).map(|p| (p, *m.get(p).unwrap())),
        |p| p.0 == (m.rows - 1) && p.1 == (m.columns - 1),
    );

    if result.is_some() {
        let (_, weight) = result.unwrap();

        return Some(weight);
    }

    None
}
