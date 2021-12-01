pub fn process(input: &str, window_len: usize) -> Option<u64> {
    let lines: Vec<u64> = input.lines().map(|l| l.parse::<u64>().unwrap()).collect();

    let mut prev = 1u64;
    let mut increases = 0u64;

    for (i, window) in lines.windows(window_len).enumerate() {
        let sum = window.into_iter().sum::<u64>();

        if i != 0 {
            if prev < sum {
                increases += 1;
            }
        }

        prev = sum;
    }

    Some(increases)
}
