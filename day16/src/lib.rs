use nom::{
    bytes::complete::{tag, take},
    multi::{many0, many_m_n},
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    literal_value: Option<u64>,
    sub_packets: Option<Vec<Packet>>,
}

fn take1(input: &str) -> IResult<&str, &str> {
    take(1u8)(input)
}

fn take4(input: &str) -> IResult<&str, &str> {
    take(4u8)(input)
}

fn take11(input: &str) -> IResult<&str, &str> {
    take(11u8)(input)
}

fn take15(input: &str) -> IResult<&str, &str> {
    take(15u8)(input)
}

fn chunk(input: &str) -> IResult<&str, &str> {
    let (input, chunk) = take4(input).unwrap();

    Ok((input, chunk))
}

fn literal(input: &str) -> IResult<&str, u64> {
    let mut lit_str = String::new();

    let (input, other) = many0(preceded(tag("1"), chunk))(input).unwrap();
    let (input, other_last) = preceded(tag("0"), chunk)(input).unwrap();

    other.iter().for_each(|&s| lit_str.push_str(s));

    lit_str.push_str(other_last);

    Ok((input, u64::from_str_radix(&lit_str, 2).unwrap()))
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    let (input, packet_version_str) = take(3usize)(input)?;
    let (input, type_id_str) = take(3usize)(input)?;

    let version = u8::from_str_radix(packet_version_str, 2).expect("Expected a binary number");
    let type_id = u8::from_str_radix(type_id_str, 2).expect("Expected a binary number");

    if type_id == 4 {
        let (input, lit) = literal(input).unwrap();

        Ok((
            input,
            Packet {
                version,
                type_id,
                literal_value: Some(lit),
                sub_packets: None,
            },
        ))
    } else {
        let (input, len_type_id) = take1(input).unwrap();

        if len_type_id == "0" {
            let (input, length_in_bits) = take15(input).unwrap();
            let sub_packet_len =
                u32::from_str_radix(length_in_bits, 2).expect("Should parse into a u32");

            let (input, sub_packets) = take::<u32, &str, ()>(sub_packet_len)(input).unwrap();

            let (_input, sub_packets_vec) = many0(parse_packet)(sub_packets).unwrap();

            Ok((
                input,
                Packet {
                    version,
                    type_id,
                    literal_value: None,
                    sub_packets: Some(sub_packets_vec),
                },
            ))
        } else {
            let (input, num_of_packets_str) = take11(input).unwrap();

            let num_of_packets =
                usize::from_str_radix(num_of_packets_str, 2).expect("Should parse into a u32");

            let (input, packets) =
                many_m_n(num_of_packets, num_of_packets, parse_packet)(input).unwrap();

            Ok((
                input,
                Packet {
                    version,
                    type_id,
                    literal_value: None,
                    sub_packets: Some(packets),
                },
            ))
        }
    }
}

fn run_packet_pt1(p: &Packet) -> usize {
    match &p.sub_packets {
        Some(sub) => (p.version as usize) + sub.iter().map(run_packet_pt1).sum::<usize>(),
        None => p.version as usize,
    }
}

fn run_packet_pt2(p: &Packet) -> usize {
    match &p.sub_packets {
        Some(sub) => {
            let mut sub_iter = sub.iter().map(run_packet_pt2);
            match p.type_id {
                0 => sub_iter.sum(),
                1 => sub_iter.product(),
                2 => sub_iter.min().unwrap(),
                3 => sub_iter.max().unwrap(),
                5 => {
                    let a = sub_iter.next().unwrap();
                    let b = sub_iter.next().unwrap();

                    if a > b {
                        return 1;
                    } else {
                        return 0;
                    }
                }
                6 => {
                    let a = sub_iter.next().unwrap();
                    let b = sub_iter.next().unwrap();

                    if a < b {
                        return 1;
                    } else {
                        return 0;
                    }
                }
                7 => {
                    let a = sub_iter.next().unwrap();
                    let b = sub_iter.next().unwrap();

                    if a == b {
                        return 1;
                    } else {
                        return 0;
                    };
                }
                _ => panic!("This shouldn't happen"),
            }
        }
        None => p.literal_value.unwrap() as usize,
    }
}

fn to_binary(input: &str) -> String {
    let f: Vec<u32> = input
        .chars()
        .map(|c| c.to_digit(16).expect("Expect a hexadecimal string"))
        .collect();

    let f_str: Vec<String> = f.iter().map(|num| format!("{:04b}", num)).collect();

    f_str.join("")
}

pub fn process(input: &str) -> Option<usize> {
    let binary_str = to_binary(input);

    let (_, packet) = parse_packet(&binary_str).unwrap();

    Some(run_packet_pt1(&packet))
}

pub fn process_pt2(input: &str) -> Option<usize> {
    let binary_str = to_binary(input);

    let (_, packet) = parse_packet(&binary_str).unwrap();

    Some(run_packet_pt2(&packet))
}
