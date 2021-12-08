use day06::process;
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

    let pt1_result = process(&input, 80).unwrap();
    let pt2_result = process(&input, 256).unwrap();

    println!("Result 1: {}", pt1_result);
    println!("Result 2: {}", pt2_result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(process("3,4,3,1,2", 80), Some(5934));
    }

    #[test]
    fn test_input_pt2() {
        assert_eq!(process("3,4,3,1,2", 256), Some(26984457539));
    }
}
