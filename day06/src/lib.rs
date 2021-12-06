use std::vec;

pub fn process(input: &str, days: u16) -> Option<usize> {
    let mut fishes: Vec<u8> = input.split(',').map(|f| f.parse::<u8>().unwrap()).collect();

    let mut new_fish: Vec<u8> = vec![];

    for curr_day in 0..days {
        for fish in fishes.iter_mut() {
            if *fish > 0 {
                *fish -= 1;
            } else {
                *fish = 6;
                new_fish.push(8);
            }
        }
        fishes.append(&mut new_fish);
        new_fish.clear();
        println!("{} - {} fish", curr_day, fishes.len());
    }
    Some(fishes.len())
}
