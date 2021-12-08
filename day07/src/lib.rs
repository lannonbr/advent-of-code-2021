pub fn process(input: &str) -> Option<u32> {
    let mut crabs: Vec<u32> = input
        .split(',')
        .map(|c| c.parse::<u32>().unwrap())
        .collect();

    crabs.sort();

    let crabs_median = crabs[crabs.len() / 2];

    let sum = crabs.iter().fold(0, |acc, &crab| {
        acc + (crab as i32 - crabs_median as i32).abs() as u32
    });

    Some(sum)
}

// NOTE: This worked for my solution, but because I am dividing 2 u32s, there likely may be an off by one error for some inputs.
pub fn process_pt2(input: &str) -> Option<u32> {
    let crabs: Vec<u32> = input
        .split(',')
        .map(|c| c.parse::<u32>().unwrap())
        .collect();

    let crabs_mean = crabs.iter().sum::<u32>() / crabs.len() as u32;

    let sum = crabs.iter().fold(0, |acc, &crab| {
        let dist = (crab as i32 - crabs_mean as i32).abs() as u32;

        acc + (1..=dist).sum::<u32>()
    });

    Some(sum)
}
