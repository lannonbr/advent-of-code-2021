use day11::{process, process_pt2};
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
    let steps = args[2].parse::<usize>().unwrap();

    let pt1_result = process(&input, 100).unwrap();
    let pt2_result = process_pt2(&input, steps).unwrap();

    println!("Result 1: {}", pt1_result);
    println!("Result 2: {}", pt2_result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            process(
                "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
                100
            ),
            Some(1656)
        );
    }

    #[test]
    fn test_nines() {
        assert_eq!(
            process(
                "11111
19991
19191
19991
11111",
                2
            ),
            Some(9)
        )
    }
}
