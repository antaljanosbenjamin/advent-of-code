use std::fs;

use common::utility::print_solution;

fn greater_than(lhs: usize, rhs: usize) -> usize {
    if lhs > rhs {
        1
    } else {
        0
    }
}

fn equal_value(lhs: usize, rhs: usize) -> usize {
    if lhs == rhs {
        1
    } else {
        0
    }
}

static mut INDENT: String = String::new();

fn parse_packet_versions(
    binary: &str,
) -> (
    usize, /*version sum*/
    usize, /*parsed length*/
    usize, /*value*/
) {
    unsafe {
        INDENT.push_str("  ");
    }
    let version = usize::from_str_radix(&binary[0..3], 2).unwrap();
    let type_id = usize::from_str_radix(&binary[3..6], 2).unwrap();
    unsafe {
        println!(
            "{}Parsing packet with version {}, type {}",
            INDENT, version, type_id
        );
    }
    let mut res = match type_id {
        4 => parse_binary_versions(&binary[6..]),
        _ => {
            let op_res = parse_operator_versions(&binary[6..]);
            match type_id {
                0 => (op_res.0, op_res.1, op_res.2.iter().sum::<usize>()),
                1 => (op_res.0, op_res.1, op_res.2.iter().product()),
                2 => (op_res.0, op_res.1, *op_res.2.iter().min().unwrap()),
                3 => (op_res.0, op_res.1, *op_res.2.iter().max().unwrap()),
                5 => (op_res.0, op_res.1, greater_than(op_res.2[0], op_res.2[1])),
                6 => (op_res.0, op_res.1, greater_than(op_res.2[1], op_res.2[0])),
                7 => (op_res.0, op_res.1, equal_value(op_res.2[1], op_res.2[0])),
                _ => panic!("not good"),
            }
        }
    };
    res.0 += version;
    res.1 += 6;
    unsafe {
        println!(
            "{}Parsed packet with version {}, type {} and value {}",
            INDENT, version, type_id, res.2
        );
        INDENT = INDENT[0..INDENT.len() - 2].to_string();
    }
    res
}

fn parse_binary_versions(
    binary: &str,
) -> (
    usize, /*version sum*/
    usize, /*parsed length*/
    usize, /*value*/
) {
    let mut parsed_len = 0;
    let mut value = 0;
    while binary.chars().nth(parsed_len) == Some('1') {
        let add = usize::from_str_radix(&binary[parsed_len + 1..parsed_len + 5], 2).unwrap();
        value += add;
        unsafe {
            println!(
                "{}  {} -> {} (sum: {})",
                INDENT,
                &binary[parsed_len + 1..parsed_len + 5],
                add,
                value
            );
        }
        value *= 16;
        parsed_len += 5;
    }
    value += usize::from_str_radix(&binary[parsed_len + 1..parsed_len + 5], 2).unwrap();
    parsed_len += 5;
    unsafe {
        println!("{}Parsed {}", INDENT, value);
    }
    (0, parsed_len, value)
}

fn parse_operator_versions(
    binary: &str,
) -> (
    usize,      /*version sum*/
    usize,      /*parsed length*/
    Vec<usize>, /*values*/
) {
    // println!(
    //     "Parsing packet with version {}, type {} and binary {}",
    //     version, type_id, binary
    // );
    let mut res = match binary.chars().next().unwrap() {
        '0' => parse_operator_versions_by_length(&binary[1..]),
        '1' => parse_operator_verions_by_subpacket_num(&binary[1..]),
        _ => panic!("Invalid number"),
    };
    res.1 += 1;
    res
}

fn parse_operator_versions_by_length(
    binary: &str,
) -> (
    usize,      /*version sum*/
    usize,      /*parsed length*/
    Vec<usize>, /*values*/
) {
    let sub_packets_len = usize::from_str_radix(&binary[0..15], 2).unwrap();
    let mut parsed_len = 15;
    let mut version_sum = 0;
    let mut values = Vec::new();
    while parsed_len != sub_packets_len + 15 {
        let res = parse_packet_versions(&binary[parsed_len..]);
        parsed_len += res.1;
        version_sum += res.0;
        values.push(res.2);
    }
    (version_sum, parsed_len, values)
}

fn parse_operator_verions_by_subpacket_num(
    binary: &str,
) -> (
    usize,      /*version sum*/
    usize,      /*parsed length*/
    Vec<usize>, /*values*/
) {
    let sub_packets_num = usize::from_str_radix(&binary[0..11], 2).unwrap();
    let mut parsed_len = 11;
    let mut version_sum = 0;
    let mut values = Vec::new();
    for _i in 0..sub_packets_num {
        let res = parse_packet_versions(&binary[parsed_len..]);
        parsed_len += res.1;
        version_sum += res.0;
        values.push(res.2);
    }
    (version_sum, parsed_len, values)
}

fn main() {
    let file_content = fs::read_to_string("2021/16/input.txt")
        .expect("Cannot read input file")
        .trim()
        .to_string();
    let binary = file_content
        .chars()
        .fold(String::new(), |mut binary_str, hex_char| {
            let num = hex_char.to_digit(16).expect("Invalid input");
            binary_str.push_str(&format!("{:04b}", num));
            binary_str
        });
    println!("{}", binary);
    let res = parse_packet_versions(&binary);
    print_solution(1, res.0);
    print_solution(1, res.2);
}
