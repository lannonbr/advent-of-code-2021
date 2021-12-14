use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, anychar, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn mapping(input: &str) -> IResult<&str, (&str, char)> {
    let (input, (left, right)) = separated_pair(alpha1, tag(" -> "), anychar)(input)?;

    Ok((input, (left, right)))
}

fn puzzle_input(input: &str) -> IResult<&str, (&str, HashMap<&str, char>)> {
    let (input, starting_line) = take_until("\n")(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let (input, mappings_vec) = separated_list1(newline, mapping)(input)?;
    let mappings = HashMap::from_iter(mappings_vec.iter().cloned());
    Ok((input, (starting_line, mappings)))
}

pub fn process(input: &str) -> Option<u64> {
    let (_, (starting_input, mappings)) = puzzle_input(input).unwrap();

    let vec: Vec<char> = starting_input.chars().collect::<Vec<char>>();

    let last_char = *vec.last().unwrap();

    let mut count_map = run_str(vec, &mappings, 10);

    let en = count_map.entry(last_char).or_default();
    *en += 1;

    let largest = count_map.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let smallest = count_map.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();

    Some(largest.1 - smallest.1)
}

fn run_str(s: Vec<char>, mappings: &HashMap<&str, char>, count: usize) -> HashMap<char, u64> {
    let mut vec = s.clone();

    for i in 0..count {
        let mut new_vec: Vec<char> = vec![];

        vec.windows(2)
            .map(|a| String::from_iter(a.iter()))
            .for_each(|a| {
                let new_char = *mappings.get(a.as_str()).unwrap();
                new_vec.push(a.chars().nth(0).unwrap());
                new_vec.push(new_char);
            });
        let last_char = *vec.last().unwrap();
        new_vec.push(last_char);

        vec = new_vec;

        let c: String = vec.iter().collect();
        println!("{}, (len {})", i, c.len());
    }
    vec.pop();

    let mut count_map: HashMap<char, u64> = HashMap::new();

    for &c in vec.iter() {
        let en = count_map.entry(c).or_default();
        *en += 1;
    }

    count_map
}

pub fn process_pt2(input: &str) -> Option<u64> {
    let (_, (starting_input, mappings)) = puzzle_input(input).unwrap();

    let vec: Vec<char> = starting_input.chars().collect::<Vec<char>>();

    let last_char = *vec.last().unwrap();

    let mut count_map: HashMap<char, u64> = HashMap::new();

    vec.windows(2)
        .map(|a| String::from_iter(a.iter()))
        .for_each(|a| {
            let mini_count = run_str(a.chars().collect::<Vec<char>>(), &mappings, 40);

            for (c, u) in mini_count {
                let en = count_map.entry(c).or_default();
                *en += u;
            }
        });

    let en = count_map.entry(last_char).or_default();
    *en += 1;

    let largest = count_map.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let smallest = count_map.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();

    dbg!(smallest, largest);

    Some(largest.1 - smallest.1)
}
