use day02::process;
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

    println!("Result 1: {}", pt1_result);
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
}
