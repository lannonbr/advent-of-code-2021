use nom::{
    bytes::complete::take_till,
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

type Board = Vec<Vec<char>>;
type Mapping = Vec<char>;

fn parse_input(input: &str) -> IResult<&str, (Mapping, Board)> {
    let (input, mapping_str): (&str, &str) = take_till(|c| c == '\n')(input)?;

    let mapping: Mapping = mapping_str.chars().collect();

    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;

    let (input, board) = separated_list1(newline, row)(input)?;

    Ok((input, (mapping, board)))
}

fn row(input: &str) -> IResult<&str, Vec<char>> {
    let (input, chars) = many1(one_of(".#"))(input)?;
    Ok((input, chars))
}

fn enhance_board(board: &Board, mapping: &Mapping, s: u32) -> Board {
    let max_size = board.len() + 2;

    let wrap_char = if mapping[0] == '#' {
        if s % 2 == 0 {
            '.'
        } else {
            '#'
        }
    } else {
        '.'
    };
    let mut expanded_board = vec![vec![wrap_char; max_size]; max_size];

    for y in 0..board.len() {
        for x in 0..board[y].len() {
            expanded_board[y + 1][x + 1] = board[y][x];
        }
    }

    let mut new_board = vec![vec![' '; max_size]; max_size];

    for y in 0..expanded_board.len() {
        for x in 0..max_size {
            let mut window: Vec<char> = vec![];

            if (x as isize - 1) < 0 || (y as isize - 1) < 0 {
                window.push(wrap_char);
            } else {
                window.push(expanded_board[y - 1][x - 1]);
            }

            if y as isize - 1 < 0 {
                window.push(wrap_char);
            } else {
                window.push(expanded_board[y - 1][x]);
            }

            if x + 1 == max_size || (y as isize - 1) < 0 {
                window.push(wrap_char);
            } else {
                window.push(expanded_board[y - 1][x + 1]);
            }

            if x as isize - 1 < 0 {
                window.push(wrap_char);
            } else {
                window.push(expanded_board[y][x - 1]);
            }

            window.push(expanded_board[y][x]);

            if x + 1 == max_size {
                window.push(wrap_char);
            } else {
                window.push(expanded_board[y][x + 1]);
            }

            if x as isize - 1 < 0 || y + 1 == max_size {
                window.push(wrap_char);
            } else {
                window.push(expanded_board[y + 1][x - 1]);
            }
            if y + 1 == max_size {
                window.push(wrap_char);
            } else {
                window.push(expanded_board[y + 1][x]);
            }
            if x + 1 == max_size || y + 1 == max_size {
                window.push(wrap_char);
            } else {
                window.push(expanded_board[y + 1][x + 1]);
            }

            let window_string = window
                .iter()
                .map(|&c| if c == '#' { '1' } else { '0' })
                .collect::<String>();

            let idx: u32 = u32::from_str_radix(&window_string, 2).unwrap();

            new_board[y][x] = mapping[idx as usize];
        }
    }

    new_board
}

pub fn process(input: &str) -> Option<u64> {
    let (_, (mapping, mut board)) = parse_input(input).unwrap();

    for i in 0..2 {
        board = enhance_board(&board, &mapping, i);
    }

    let final_count = board.iter().flatten().filter(|&c| *c == '#').count();

    Some(final_count as u64)
}

pub fn process_pt2(input: &str) -> Option<u64> {
    let (_, (mapping, mut board)) = parse_input(input).unwrap();

    for i in 0..50 {
        board = enhance_board(&board, &mapping, i);
    }

    let final_count = board.iter().flatten().filter(|&c| *c == '#').count();

    Some(final_count as u64)
}
