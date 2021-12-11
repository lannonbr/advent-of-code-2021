use std::collections::HashMap;

pub fn process(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;

    let illegal_map = HashMap::from([(')', 3), ('}', 1197), (']', 57), ('>', 25137)]);

    lines.iter().for_each(|&l| {
        match find_illegal_char(l) {
            Some(c) => {
                sum += illegal_map.get(&c).unwrap();
            }
            None => {}
        };
    });

    Some(sum)
}

pub fn process_pt2(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();

    let incomplete_lines: Vec<&str> = lines
        .into_iter()
        .filter(|&l| find_illegal_char(l).is_none())
        .collect();

    let mut scores: Vec<u64> = vec![];

    for line in incomplete_lines {
        let mut stack: Vec<char> = vec![];

        for c in line.chars() {
            if is_open(c) {
                stack.push(c);
            }
            if is_closed(c) {
                stack.pop();
            }
        }

        stack.reverse();

        let score_map = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

        let mut score = 0;
        for s in stack {
            score *= 5;
            score += score_map.get(&s)?;
        }

        scores.push(score);
    }

    scores.sort();

    Some(scores[scores.len() / 2])
}

fn is_open(c: char) -> bool {
    "({[<".contains(c)
}

fn is_closed(c: char) -> bool {
    ")}]>".contains(c)
}

fn find_illegal_char(line: &str) -> Option<char> {
    let mut stack: Vec<char> = vec![];

    let matching = HashMap::from([('(', ')'), ('{', '}'), ('[', ']'), ('<', '>')]);

    for c in line.chars() {
        if is_open(c) {
            stack.push(c);
        }
        if is_closed(c) {
            let last_char = stack[stack.len() - 1];

            if c != *matching.get(&last_char)? {
                return Some(c);
            } else {
                stack.pop();
            }
        }
    }

    None
}
