use day12::{process, process_pt2};
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
                "start-A
start-b
A-c
A-b
b-d
A-end
b-end"
            ),
            Some(10)
        );
    }

    #[test]
    fn test_input_pt2() {
        assert_eq!(
            process_pt2(
                "start-A
start-b
A-c
A-b
b-d
A-end
b-end"
            ),
            Some(36)
        );
    }
}
