// const INPUTS: &str = "A0016C880162017C3686B18A3D4780";
const INPUTS: &str = include_str!("./inputs/day16.txt");

use std::collections::VecDeque;
use std::iter::FromIterator;

pub fn run() {
    let mut packets = parse_inputs(INPUTS.trim());
    let answer = solve(&mut packets);
    println!("answer: {:?}", answer);
}

type Packets = VecDeque<char>;

fn parse_inputs(inputs: &str) -> Packets {
    let mut bits = String::new();
    for c in inputs.chars() {
        match c {
            '0' => bits.push_str("0000"),
            '1' => bits.push_str("0001"),
            '2' => bits.push_str("0010"),
            '3' => bits.push_str("0011"),
            '4' => bits.push_str("0100"),
            '5' => bits.push_str("0101"),
            '6' => bits.push_str("0110"),
            '7' => bits.push_str("0111"),
            '8' => bits.push_str("1000"),
            '9' => bits.push_str("1001"),
            'A' => bits.push_str("1010"),
            'B' => bits.push_str("1011"),
            'C' => bits.push_str("1100"),
            'D' => bits.push_str("1101"),
            'E' => bits.push_str("1110"),
            'F' => bits.push_str("1111"),
            _ => unreachable!(),
        }
    }
    VecDeque::from_iter(bits.chars())
}

fn solve(mut s: &mut Packets) -> (u64, u64) {
    let mut version_bits = String::new();
    for _ in 0..3 {
        version_bits.push(s.pop_front().unwrap());
    }

    let mut type_id_bits = String::new();
    for _ in 0..3 {
        type_id_bits.push(s.pop_front().unwrap());
    }

    let mut version_sum = u64::from_str_radix(&version_bits, 2).unwrap();

    match type_id_bits.as_ref() {
        // literal
        "100" => {
            let _number = solve_literal(&mut s);
        }
        // operator
        _ => {
            let (version, _numbers) = solve_operator(&mut s);
            version_sum += version;
        }
    }
    (version_sum, 0)
}

fn solve_literal(s: &mut Packets) -> u64 {
    let mut number_bits = String::new();

    loop {
        let first_bit = s.pop_front().unwrap();

        for _ in 0..4 {
            number_bits.push(s.pop_front().unwrap());
        }

        if first_bit == '0' {
            break;
        }
    }

    u64::from_str_radix(&number_bits, 2).unwrap()
}

fn solve_operator(s: &mut Packets) -> (u64, Vec<u64>) {
    let length_type_id = s.pop_front();

    match length_type_id {
        Some('0') => solve_operator_type_0(s),
        Some('1') => solve_operator_type_1(s),
        _ => unreachable!(),
    }
}

fn solve_operator_type_0(s: &mut VecDeque<char>) -> (u64, Vec<u64>) {
    let mut length_bits = String::new();
    for _ in 0..15 {
        length_bits.push(s.pop_front().unwrap());
    }
    let length = usize::from_str_radix(&length_bits, 2).unwrap();

    let mut version_sum = 0;
    let mut values = vec![];
    let start = s.len();

    loop {
        let (version, value) = solve(s);
        version_sum += version;
        values.push(value);

        let now = s.len();
        if start - now >= length {
            break;
        }
    }

    (version_sum, values)
}

fn solve_operator_type_1(s: &mut VecDeque<char>) -> (u64, Vec<u64>) {
    let mut length_bits = String::new();
    for _ in 0..11 {
        length_bits.push(s.pop_front().unwrap());
    }
    let length = usize::from_str_radix(&length_bits, 2).unwrap();

    let mut version_sum = 0;
    let mut values = vec![];

    for _ in 0..length {
        let (version, value) = solve(s);
        version_sum += version;
        values.push(value);
    }

    (version_sum, values)
}
