use pathfinding::prelude::{dijkstra, Matrix};

pub fn process(input: &str) -> Option<u32> {
    let rows = input.lines().collect::<Vec<&str>>().len();
    let v: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let flat_v: Vec<u32> = v.iter().flatten().map(|c| *c).collect();

    // dbg!(&v);

    let m = Matrix::from_vec(rows, rows, flat_v).unwrap();

    // let iter = m.neighbours(false).map(|i| (i, m.get(i)));

    let result = dijkstra(
        &(0, 0),
        |p| m.neighbours(*p, false).map(|p| (p, *m.get(p).unwrap())),
        |p| p.0 == (m.rows - 1) && p.1 == (m.columns - 1),
    );

    if result.is_some() {
        let (path, weight) = result.unwrap();

        return Some(weight);
    }

    None
}

pub fn process_pt2(input: &str) -> Option<u32> {
    Some(5)
}
