use day02::{process, process_pt2};
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

    let pt1_result = process(&input);
    let pt2_result = process_pt2(&input);

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
                "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            ),
            150
        )
    }

    #[test]
    fn test_input_pt2() {
        assert_eq!(
            process_pt2(
                "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            ),
            900
        )
    }
}
