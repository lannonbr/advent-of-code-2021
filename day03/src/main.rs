use day03::process;
use day03::process_pt2;
use std::env;
use std::fs;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Please specify a file as an argument",
        ));
    }

    let input = fs::read_to_string(&args[1])?;

    let pt1_result = process(&input).unwrap();
    let pt2_result = process_pt2(&input).unwrap();

    println!("Result 1: {}", pt1_result);
    println!("Result 2: {}", pt2_result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use day03::process_pt2;

    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            process(
                "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            ),
            Some(198)
        )
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            process_pt2(
                "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            ),
            Some(230)
        )
    }
}
