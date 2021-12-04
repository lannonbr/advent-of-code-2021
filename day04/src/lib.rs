use std::{collections::HashMap, fmt::Display};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref WIN_LOCATIONS: Vec<Vec<(usize, usize)>> = {
        let mut wins_locations: Vec<Vec<(usize, usize)>> = vec![];

        for i in 0..5 {
            let mut row: Vec<(usize, usize)> = vec![];
            let mut col: Vec<(usize, usize)> = vec![];
            for j in 0..5 {
                row.push((i, j));
                col.push((j, i));
            }

            wins_locations.push(row);
            wins_locations.push(col);
        }

        wins_locations
    };
}

struct Board {
    board: Vec<Vec<(u64, bool)>>,
    solved: bool,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.board[i][j].1 {
                    f.write_str(&format!("{}c ", self.board[i][j].0)).unwrap();
                } else {
                    f.write_str(&format!("{} ", self.board[i][j].0)).unwrap();
                }
            }
            f.write_str("\n").unwrap()
        }
        f.pad("")
    }
}

impl Board {
    fn new(l: &str) -> Board {
        let board: Vec<Vec<(u64, bool)>> = l
            .split("\n")
            .map(|s| {
                let nums: Vec<(u64, bool)> = s
                    .split_whitespace()
                    .map(|num| (num.parse::<u64>().unwrap(), false))
                    .collect();
                nums
            })
            .collect();
        Board {
            board,
            solved: false,
        }
    }
    fn hit(&mut self, i: usize, j: usize) {
        self.board[i][j].1 = true
    }

    fn check_for_bingo(&self) -> bool {
        let f = &WIN_LOCATIONS;
        for list in f.iter() {
            'inner: for (e, (i, j)) in list.iter().enumerate() {
                let on = self.board.iter().nth(*i).unwrap().iter().nth(*j).unwrap().1;
                if on == false {
                    break 'inner;
                }
                if e == list.iter().len() - 1 {
                    return true;
                }
            }
        }
        false
    }
    fn sum_unmarked(&self) -> u64 {
        let mut sum: u64 = 0;
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.board[i][j].1 == false {
                    sum += self.board[i][j].0
                }
            }
        }
        sum
    }
}

pub fn process(input: &str) -> Option<u64> {
    let things: Vec<&str> = input.split("\n\n").collect();

    let numbers: Vec<u64> = String::from(things[0])
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut boards: Vec<Board> = things[1..]
        .to_vec()
        .iter()
        .map(|&l| Board::new(l))
        .collect();

    for &num in numbers.iter() {
        for board in boards.iter_mut() {
            for i in 0..board.board.len() {
                for j in 0..board.board[i].len() {
                    if board.board[i][j].0 == num {
                        board.hit(i, j)
                    }
                }
            }
            let res = board.check_for_bingo();
            if res {
                let sum = board.sum_unmarked();
                board.solved = true;

                return Some(sum * num);
            }
        }
    }

    None
}

pub fn process_pt2(input: &str) -> Option<u64> {
    let things: Vec<&str> = input.split("\n\n").collect();

    let numbers: Vec<u64> = String::from(things[0])
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut boards: Vec<Board> = things[1..]
        .to_vec()
        .iter()
        .map(|&l| Board::new(l))
        .collect();

    let mut wins: HashMap<usize, (usize, u64)> = HashMap::new();

    for (a, &num) in numbers.iter().enumerate() {
        'board_loop: for (e, board) in boards.iter_mut().enumerate() {
            if board.solved {
                continue 'board_loop;
            }
            for i in 0..board.board.len() {
                for j in 0..board.board[i].len() {
                    if board.board[i][j].0 == num {
                        board.hit(i, j)
                    }
                }
            }
            let res = board.check_for_bingo();
            if res {
                if wins.get(&e).is_none() {
                    board.solved = true;
                    wins.insert(e, (a, num));
                }
            }
        }
    }

    let (&e, last_board_details) = wins
        .iter()
        .max_by(|x, y| {
            let x1 = x.1;
            let y1 = y.1;
            let x1_0 = x1.0;
            let y1_0 = y1.0;
            x1_0.cmp(&y1_0)
        })
        .unwrap();

    let num = last_board_details.1;
    let last_board = &boards[e];

    Some(last_board.sum_unmarked() * num)
}
