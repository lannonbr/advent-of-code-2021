pub fn process(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();

    let mut gamma_vec: Vec<&str> = vec![];
    let mut epsilon_vec: Vec<&str> = vec![];

    let len = &lines.iter().nth(0).unwrap().len();
    for i in 0..*len {
        let (zeros, ones): (i32, i32) = lines.iter().fold((0, 0), |mut acc, &x| {
            if x.chars().nth(i).unwrap() == '0' {
                acc.0 += 1;
            } else {
                acc.1 += 1;
            }
            acc
        });

        if zeros > ones {
            gamma_vec.push("0");
            epsilon_vec.push("1");
        } else {
            gamma_vec.push("1");
            epsilon_vec.push("0");
        }
    }

    let gamma = u64::from_str_radix(&gamma_vec.join(""), 2).unwrap();
    let epsilon = u64::from_str_radix(&epsilon_vec.join(""), 2).unwrap();

    Some(gamma * epsilon)
}

pub fn process_pt2(input: &str) -> Option<u64> {
    let mut oxy_lines: Vec<&str> = input.lines().collect();
    let mut co2_lines: Vec<&str> = oxy_lines.clone();

    let o_len = &oxy_lines.iter().nth(0).unwrap().len();
    for i in 0..*o_len {
        let (zeros, ones): (i32, i32) = oxy_lines.iter().fold((0, 0), |mut acc, &x| {
            if x.chars().nth(i).unwrap() == '0' {
                acc.0 += 1;
            } else {
                acc.1 += 1;
            }
            acc
        });

        if zeros > ones {
            oxy_lines = oxy_lines
                .into_iter()
                .filter(|&l| l.chars().nth(i).unwrap() == '0')
                .collect();
        } else {
            oxy_lines = oxy_lines
                .into_iter()
                .filter(|&l| l.chars().nth(i).unwrap() == '1')
                .collect();
        }

        if oxy_lines.len() == 1 {
            break;
        }
    }

    let c_len = &co2_lines.iter().nth(0).unwrap().len();
    for i in 0..*c_len {
        let (zeros, ones): (i32, i32) = co2_lines.iter().fold((0, 0), |mut acc, &x| {
            if x.chars().nth(i).unwrap() == '0' {
                acc.0 += 1;
            } else {
                acc.1 += 1;
            }
            acc
        });

        if zeros > ones {
            co2_lines = co2_lines
                .into_iter()
                .filter(|&l| l.chars().nth(i).unwrap() == '1')
                .collect();
        } else {
            co2_lines = co2_lines
                .into_iter()
                .filter(|&l| l.chars().nth(i).unwrap() == '0')
                .collect();
        }

        if co2_lines.len() == 1 {
            break;
        }
    }

    let oxy = u64::from_str_radix(&oxy_lines[0], 2).unwrap();
    let co2 = u64::from_str_radix(&co2_lines[0], 2).unwrap();

    Some(oxy * co2)
}
