use day09::{process, process_pt2};
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
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            process(
                "2199943210
3987894921
9856789892
8767896789
9899965678"
            ),
            Some(15)
        );
    }

    #[test]
    fn test_input_pt2() {
        assert_eq!(
            process_pt2(
                "2199943210
3987894921
9856789892
8767896789
9899965678"
            ),
            Some(1134)
        );
    }
}
