use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, anychar, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn mapping(input: &str) -> IResult<&str, ((char, char), char)> {
    let (input, (left, right)) = separated_pair(alpha1, tag(" -> "), anychar)(input)?;

    Ok((
        input,
        (
            (left.chars().nth(0).unwrap(), left.chars().nth(1).unwrap()),
            right,
        ),
    ))
}

fn puzzle_input(input: &str) -> IResult<&str, (&str, HashMap<(char, char), char>)> {
    let (input, starting_line) = take_until("\n")(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let (input, mappings_vec) = separated_list1(newline, mapping)(input)?;
    let mappings = HashMap::from_iter(mappings_vec.iter().cloned());
    Ok((input, (starting_line, mappings)))
}

fn run_str(s: Vec<char>, mappings: &HashMap<(char, char), char>, count: usize) -> u64 {
    let vec = s.clone();

    let last_char = *vec.last().unwrap();

    let mut state: HashMap<(char, char), u64> = HashMap::new();

    for a in vec.windows(2) {
        state
            .entry((a[0], a[1]))
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    for _ in 0..count {
        let mut new_state: HashMap<(char, char), u64> = HashMap::new();

        for (ident, current_count) in state.iter() {
            let new_char = *mappings.get(&ident).unwrap();
            new_state
                .entry((ident.0, new_char))
                .and_modify(|c| *c += current_count)
                .or_insert(*current_count);
            new_state
                .entry((new_char, ident.1))
                .and_modify(|c| *c += current_count)
                .or_insert(*current_count);
        }

        state = new_state;
    }

    let mut count_map: HashMap<char, u64> = HashMap::new();

    for a in state.iter().map(|(a, c)| (a.0, c)) {
        count_map
            .entry(a.0)
            .and_modify(|f| *f += *a.1)
            .or_insert(*a.1);
    }

    count_map
        .entry(last_char)
        .and_modify(|f| *f += 1)
        .or_insert(1);

    let largest = count_map.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let smallest = count_map.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();

    largest.1 - smallest.1
}

pub fn process(input: &str) -> Option<u64> {
    let (_, (starting_input, mappings)) = puzzle_input(input).unwrap();

    let s: Vec<char> = starting_input.chars().collect::<Vec<char>>();

    Some(run_str(s, &mappings, 10))
}

pub fn process_pt2(input: &str) -> Option<u64> {
    let (_, (starting_input, mappings)) = puzzle_input(input).unwrap();

    let s = starting_input.chars().collect::<Vec<char>>();

    Some(run_str(s, &mappings, 40))
}
