use day01::process;
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

    let pt1_result = process(&input, 1).unwrap();
    let pt2_result = process(&input, 3).unwrap();

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
                "199
200
208
210
200
207
240
269
260
263",
                1
            ),
            Some(7)
        )
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            process(
                "199
200
208
210
200
207
240
269
260
263",
                3
            ),
            Some(5)
        )
    }
}
