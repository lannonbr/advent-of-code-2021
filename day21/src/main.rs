use day21::{process, process_pt2};
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
                "Player 1 starting position: 4
Player 2 starting position: 8"
            ),
            Some(739785)
        );
    }

    //     #[test]
    //     fn test_input_pt2() {
    //         assert_eq!(
    //             process_pt2(
    //                 "Player 1 starting position: 4
    // Player 2 starting position: 8"
    //             ),
    //             Some(444356092776315)
    //         );
    //     }
}

/*
111 - 3
112 - 4
113 - 5
121 - 4
122 - 5
123 - 6
131 - 5
132 - 6
133 - 7

211 - 4
212 - 5
213 - 6
221 - 5
222 - 6
223 - 7
231 - 6
232 - 7
233 - 8

311 - 5
312 - 6
313 - 7
321 - 6
322 - 7
323 - 8
331 - 7
332 - 8
333 - 9

[(3,1), (4,3), (5,6), (6,7), (7,6), (8,3), (9,1)]

3 - 1 times
4 - 3 times
5 - 6 times
6 - 7 times
7 - 6 times
8 - 3 times
9 - 1 time

*/
