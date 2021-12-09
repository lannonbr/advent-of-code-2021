pub fn process(input: &str) -> Option<u32> {
    let half_two: Vec<Vec<(&str, usize)>> = input
        .lines()
        .map(|l| {
            l.split('|')
                .nth(1)
                .unwrap()
                .trim()
                .split(' ')
                .map(|s| (s, s.len()))
                .collect()
        })
        .collect();

    let mut sum = 0;

    for line in &half_two {
        for (_, size) in line {
            dbg!(size);
            if *size == 2 || *size == 3 || *size == 4 || *size == 7 {
                sum += 1;
            }
        }
    }

    Some(sum)
}

pub fn process_pt2(input: &str) -> Option<u32> {
    Some(0)
}
