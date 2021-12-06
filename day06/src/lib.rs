use std::collections::HashMap;

pub fn process(input: &str, days: u16) -> Option<u64> {
    let fishes: Vec<u8> = input.split(',').map(|f| f.parse::<u8>().unwrap()).collect();
    let mut fish_map: HashMap<u8, u64> = HashMap::new();

    for fish in fishes {
        let entry = fish_map.entry(fish).or_insert(0);
        *entry += 1;
    }

    for _ in 0..days {
        let mut new_map: HashMap<u8, u64> = HashMap::new();
        for (k, v) in fish_map.iter() {
            if *k == 0 {
                let entry = new_map.entry(6).or_insert(0);
                *entry += v;
                let entry2 = new_map.entry(8).or_insert(0);
                *entry2 += v;
            } else {
                let entry = new_map.entry(k-1).or_insert(0);
                *entry += v;
            }
        }
        fish_map = new_map;
    }

    
    let sum = fish_map.values().sum();

    Some(sum)
}
